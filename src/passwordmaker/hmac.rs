use crate::Hasher;

pub(super) fn hmac<T, K, M>(key : K, data : M) -> T::Output
    where T : Hasher,
    T::Output : AsRef<[u8]>,
    K : Iterator<Item=u8> + Clone,
    M : Iterator<Item=u8>,
{
    let key_len = key.clone().count();
    let key =  if key_len > 64 {
        KeyOrHash::from_hash(T::hash(&key.collect::<Vec<_>>()))
    } else {
        KeyOrHash::from_key(key)
    };
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

enum KeyOrHash<K: Iterator<Item=u8>, H: AsRef<[u8]>> {
    Key(K),
    Hash{
        hash : H,
        idx : usize
    }
}

impl<K: Iterator<Item=u8>, H: AsRef<[u8]>> KeyOrHash<K, H>{
    fn from_key(key : K) -> Self {
        Self::Key(key)
    }
    fn from_hash(hash : H) -> Self {
        Self::Hash { hash, idx: 0 }
    }
}

impl<K: Iterator<Item=u8>, H: AsRef<[u8]>> Iterator for KeyOrHash<K, H>{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            KeyOrHash::Key(k) => k.next(),
            KeyOrHash::Hash { hash: owned, idx } => {
                *idx += 1;
                owned.as_ref().get(*idx-1).copied()
            },
        }
    }
}