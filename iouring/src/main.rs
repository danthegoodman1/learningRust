use std::alloc::{alloc, Layout};
use std::os::unix::fs::OpenOptionsExt;
use tokio_uring::fs::{File, OpenOptions};

const BLOCK_SIZE: usize = 4096; // Typical block size, adjust as needed

struct NvmeDevice {
    fd: Option<File>,
}

#[repr(align(BLOCK_SIZE))]
struct AlignedPage([u8; BLOCK_SIZE]);

impl NvmeDevice {
    pub async fn new(device_path: &str) -> std::io::Result<Self> {
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_DIRECT)
            .open(device_path)
            .await?;

        Ok(Self { fd: Some(fd) })
    }

    /**
     * Read a block from the device.
     *
     * Returns a tuple containing the read data and the number of bytes read.
     */
    pub async fn read_block(&mut self, offset: u64) -> std::io::Result<(AlignedPage, usize)> {
        // Create a vec with the correct capacity
        let mut buf = AlignedPage([0u8; BLOCK_SIZE]);

        // Perform the read operation
        let (res, vec) = self.fd.as_mut().unwrap().read_at(buf, offset).await;
        let n = res?;

        Ok((vec, n))
    }

    /**
     * Write a block to the device.
     * It's assumed that the data is already aligned to the block size.
     */
    pub async fn write_block(
        &mut self,
        offset: u64,
        data: AlignedPage,
    ) -> std::io::Result<()> {
        let (res, _) = self
            .fd
            .as_mut()
            .unwrap()
            .write_at(data, offset)
            .submit()
            .await;
        res?;

        Ok(())
    }
}

// Could instead just make the caller responsible for closing the file
impl Drop for NvmeDevice {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                if let Some(fd) = self.fd.take() {
                    fd.close().await.unwrap();
                    self.fd = None;
                }
            });
        });
        // Or use wait_for_destruction and let this happen async (if still need blocking, otherwise just spawn close)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    // Create a new device instance
    let mut device = NvmeDevice::new("/dev/nvme0n1").await?;

    // Example: Read from offset 0
    let (data, n) = device.read_block(0).await?;
    println!("Read {} bytes", n);

    // Example: Write to offset 0
    let write_data = vec![0u8; BLOCK_SIZE];
    device.write_block(0, write_data).await?;
    println!("Write completed");

    Ok(())
}
