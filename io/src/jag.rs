use crate::{bz2_decompress, Packet};

pub struct JagFile {
    pub file_count: usize,
    pub file_hashes: Vec<i32>,
    pub file_unpacks: Vec<i32>,
    pub file_packs: Vec<i32>,
    pub file_offsets: Vec<usize>,
    pub data: Vec<u8>,
    pub unpacked: bool,
}

impl JagFile {
    /// Calculates the hash for a given file name.
    ///
    /// This function computes a unique hash for a file name. The hash is computed by first converting the
    /// name to uppercase, then iterating through each character in the name, performing a series of operations
    /// (multiplying by 61 and adding the character code) to calculate the final hash value.
    ///
    /// # Arguments
    /// - `name`: The name of the file whose hash will be computed.
    ///
    /// # Return
    /// - Returns an `i32` value representing the computed hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// use io::JagFile;
    ///
    /// assert_eq!(22834782, JagFile::hash("gnomeball_buttons.dat"));
    /// ```
    /// ----
    /// ```rust
    /// use io::JagFile;
    ///
    /// assert_eq!(-288954319, JagFile::hash("headicons.dat"));
    /// ```
    /// ----
    /// ```rust
    /// use io::JagFile;
    ///
    /// assert_eq!(-1502153170, JagFile::hash("hitmarks.dat"));
    /// ```
    pub fn hash(name: &str) -> i32 {
        let mut hash: i32 = 0;
        for ch in name.as_bytes() {
            hash = hash
                .wrapping_mul(61)
                .wrapping_add(ch.to_ascii_uppercase() as i32 - 32);
        }
        return hash;
    }

    /// Creates a new `JagFile` instance from raw byte data.
    ///
    /// This method takes a raw byte vector representing the contents of a `.jag` file, reads the metadata (such as
    /// file count, hashes, and offsets), and returns a `JagFile` object that can be used to extract the files within it.
    ///
    /// The byte data may be compressed or unpacked. If the data is compressed, it will be decompressed before being
    /// processed.
    ///
    /// # Arguments
    /// - `bytes`: A `Vec<u8>` containing the raw byte data of the `JagFile`.
    ///
    /// # Return
    /// - Returns a new `JagFile` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use io::JagFile;
    ///
    /// let jag: JagFile = JagFile::new(std::fs::read("../data/pack/client/interface").unwrap());
    /// assert_eq!(1, jag.file_count);
    /// assert_eq!(8297314, jag.file_hashes[0]);
    /// ```
    /// ----
    /// ```rust
    /// use io::JagFile;
    ///
    /// let jag: JagFile = JagFile::new(std::fs::read("../data/pack/client/config").unwrap());
    /// assert_eq!(16, jag.file_count);
    /// assert_eq!(886159288, jag.file_hashes[0]);
    /// ```
    pub fn new(bytes: Vec<u8>) -> JagFile {
        let mut buf: Packet = Packet::from(bytes);
        let unpacked: i32 = buf.g3();
        let packed: i32 = buf.g3();

        let mut decompressed: bool = false;

        if packed != unpacked {
            buf = Packet::from(bz2_decompress(buf.data, unpacked as usize, true, 6));
            decompressed = true;
        }

        let file_count: usize = buf.g2() as usize;

        let mut file_hashes: Vec<i32> = vec![0; file_count];
        let mut file_unpacks: Vec<i32> = vec![0; file_count];
        let mut file_packs: Vec<i32> = vec![0; file_count];
        let mut file_offsets: Vec<usize> = vec![0; file_count];

        let mut pos: usize = buf.pos + file_count * 10;
        for index in 0..file_count {
            file_hashes[index] = buf.g4s();
            file_unpacks[index] = buf.g3();
            file_packs[index] = buf.g3();
            file_offsets[index] = pos;
            pos += file_packs[index] as usize;
        }

        return JagFile {
            file_count,
            file_hashes,
            file_unpacks,
            file_packs,
            file_offsets,
            data: buf.data,
            unpacked: decompressed,
        };
    }

    /// Reads a file from the `JagFile` by its name.
    ///
    /// This method computes the hash for the provided file name and attempts to find the file in the `JagFile` by
    /// matching the hash with the stored file hashes. If the file is found, it returns the file's data wrapped in
    /// a `Packet`. If the file is not found, it returns `None`.
    ///
    /// # Arguments
    /// - `name`: The name of the file to read.
    ///
    /// # Return
    /// - Returns an `Option<Packet>`. If the file is found, it returns `Some(Packet)` containing the file's data.
    /// - If the file is not found, it returns `None`.
    ///
    /// # Example
    /// ```rust
    /// use io::JagFile;
    ///
    /// let jag: JagFile = JagFile::new(std::fs::read("../data/pack/client/media").unwrap());
    /// assert!(jag.read("gnomeball_buttons.dat").is_some());
    /// ```
    pub fn read(&self, name: &str) -> Option<Packet> {
        let hash: i32 = JagFile::hash(name);
        return self
            .file_hashes
            .iter()
            .position(|&file_hash| file_hash == hash)
            .and_then(|index| self.get(index));
    }

    /// Retrieves a file from the `JagFile` by its index.
    ///
    /// This method retrieves the file at the specified index in the `JagFile`. It checks if the file index is
    /// valid and if the file's data is within bounds. If the file is found and the data is valid, it returns the
    /// file as a `Packet`. If the file is not found or is out of bounds, it returns `None`.
    ///
    /// # Arguments
    /// - `index`: The index of the file to retrieve.
    ///
    /// # Return
    /// - Returns an `Option<Packet>`. If the file exists and the data is valid, it returns `Some(Packet)` with
    ///   the file's data. Otherwise, it returns `None`.
    ///
    /// # Example
    /// ```rust
    /// use io::JagFile;
    ///
    /// let jag: JagFile = JagFile::new(std::fs::read("../data/pack/client/interface").unwrap());
    /// let packet = jag.get(0);
    /// assert!(packet.is_some());
    /// ```
    pub fn get(&self, index: usize) -> Option<Packet> {
        if index >= self.file_count {
            return None;
        }

        if self.file_offsets[index] >= self.data.len() {
            return None;
        }

        let start: usize = self.file_offsets[index];
        let end: usize = start + self.file_packs[index] as usize;
        return if self.unpacked {
            Some(Packet::from(self.data[start..end].to_vec()))
        } else {
            Some(Packet::from(bz2_decompress(
                self.data[start..end].to_vec(),
                self.file_unpacks[index] as usize,
                true,
                0,
            )))
        };
    }
}
