use crate::Hasher;

pub(super) fn hmac<T, K, M>(key : K, data : M) -> T::Output
    where T : Hasher,
    T::Output : AsRef<[u8]>,
    K : Iterator<Item=u8>,
    M : Iterator<Item=u8>,
{
    let key = key.collect::<Vec<_>>();
    let key_hash = if key.len() > 64 { Some(T::hash(&key)) } else { None };
    let key = key_hash.as_ref().map(T::Output::as_ref).map(<&[u8]>::into_iter).unwrap_or_else(|| (&key).into_iter()).copied();

    let key = key.chain(std::iter::repeat(0)); //if key[i] does not exist, use 0 instead.

    let mut inner_pad = [0u8;64];
    let mut outer_pad = [0u8;64];

    let pads = inner_pad.iter_mut().zip(outer_pad.iter_mut());
    for ((i,o),k) in pads.zip(key) {
        *i = k ^ 0x36;
        *o = k ^ 0x5C;
    }

    let hash = T::hash(&inner_pad.iter().copied().chain(data).collect::<Vec<_>>());
    T::hash(&outer_pad.iter().chain(hash.as_ref().iter()).copied().collect::<Vec<_>>())
}