use crate::Hasher;

pub(super) fn hmac<T, M>(key : &[u8], data : M) -> T::Output
    where T : Hasher,
    T::Output : AsRef<[u8]>,
    M : Iterator<Item=u8>,
{
    //Sorry for this uglyness. key_hash is an Option because we don't want to compute it if we don't need it, but
    //we also want to be able to reference it in case it's needed.
    let key_hash = if key.len() > 64 { Some(T::hash(&key)) } else { None };
    let key = key_hash.as_ref().map(T::Output::as_ref).map(<&[u8]>::into_iter)
        .unwrap_or_else(|| (&key).into_iter()).copied();

    let key = key
        .chain(std::iter::repeat(0)) //if key[i] does not exist, use 0 instead.
        .take(64); //and the pads have 64 bytes

    let inner_pad = key.clone().map(|k| k ^ 0x36);
    let outer_pad = key.map(|k| k ^ 0x5C);

    let hash = T::hash(&inner_pad.chain(data).collect::<Vec<_>>());
    T::hash(&outer_pad.chain(hash.as_ref().iter().copied()).collect::<Vec<_>>())
}