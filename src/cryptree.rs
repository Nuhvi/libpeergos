//! A cryptree node controls read and write access to a directory or file.

/// A cryptree node controls read and write access to a directory or file.
///
/// A directory contains the following distinct symmetric read keys {base, parent}, and file contains {base == parent, data}
/// A directory or file also has a single base symmetric write key
///
/// A link node is a special node that behaves like a directory with a single child, and contains only the filename.
/// These are used when granting write access to prevent the recipient from being able to rename the file/dir to
/// potentially clash with a sibling that they cannot see. This means you cannot rename something unless you have write
/// access to the parent directory, which is in line with unix et al.
///
/// The serialized encrypted form stores a link from the base key to the other key.
/// For a directory, the base key encrypts the links to child directories and files. For a file the datakey encrypts the
/// file's data. The parent key encrypts the link to the parent directory's parent key and the metadata (FileProperties).
///
/// There are three network visible components to the serialization:
/// 1) A fixed size block encrypted with the base key, containing the second key (parent or data key), the location of
///       the next chunk, and an optional symmetric link to a signing pair
/// 2) A fragmented padded cipher text, padded to a multiple of 4096,
///       containing the relative child links of a directory, or the data of a file chunk
/// 3) A padded cipher text (to a multiple of 16 bytes) of an optional relative parent link, and file properties
///       The parent link is present on the first chunk of all files and directories except your home directory
///
#[derive(Debug)]
pub struct CryptreeNode<P> {
    previous_version: Option<P>,
    is_directory: bool,
}

impl<P: Clone> CryptreeNode<P> {
    pub fn new(is_directory: bool, previous_version: Option<P>) -> Self {
        Self {
            is_directory,
            previous_version,
        }
    }

    /// Create an in memory [space](../design/write_space.md).
    pub fn new_in_memory_space() -> Self {
        Self {
            previous_version: None,
            is_directory: true,
        }
    }

    // === Getters ===

    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    pub fn previous_version(&self) -> Option<&P> {
        self.previous_version.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Pointer {
        user_id: u32,
        write_space: u16,
        version: u32,
    }

    #[test]
    fn basic() {
        let fs: CryptreeNode<Pointer> = CryptreeNode::new_in_memory_space();

        dbg!(fs);

        assert!(true);
    }
}
