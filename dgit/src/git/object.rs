use std::io::Read;

/// An object ID.
pub(crate) trait ObjectID {}

pub(crate) trait ObjectReader: Read {}

struct Object<D, ID> {
    data: D,
    id: Option<ID>,
}

impl<D, ID> Read for Object<D, ID>
where
    D: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }
}
