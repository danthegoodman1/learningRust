use std::os::unix::fs::OpenOptionsExt;
use tokio_uring::buf::{IoBuf, IoBufMut};
use tokio_uring::fs::{File, OpenOptions};

const BLOCK_SIZE: usize = 4096; // Typical block size, adjust as needed

struct NvmeDevice {
    fd: Option<File>,
}

#[repr(align(4096))]
struct AlignedPage([u8; BLOCK_SIZE]);

unsafe impl IoBuf for AlignedPage {
    fn stable_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    fn bytes_init(&self) -> usize {
        BLOCK_SIZE
    }

    fn bytes_total(&self) -> usize {
        BLOCK_SIZE
    }
}

unsafe impl IoBufMut for AlignedPage {
    fn stable_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    unsafe fn set_init(&mut self, pos: usize) {
        debug_assert!(pos <= BLOCK_SIZE);
    }
}

impl NvmeDevice {
    pub async fn new(device_path: &str) -> std::io::Result<Self> {
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            // .custom_flags(libc::O_DIRECT) // Direct IO is not supported with tokio-uring!
            .open(device_path)
            .await?;

        Ok(Self { fd: Some(fd) })
    }

    /**
     * Read a block from the device.
     *
     * Returns a tuple containing the read data and the number of bytes read.
     */
    pub async fn read_block(&mut self, offset: u64) -> std::io::Result<AlignedPage> {
        // Create a vec with the correct capacity
        // let page = AlignedPage([0; BLOCK_SIZE]);
        let page = vec![0; BLOCK_SIZE];
        // Perform the read operation
        let (res, page) = self.fd.as_mut().unwrap().read_exact_at(page, offset).await;
        res?;

        // Ok(page)
        Ok(AlignedPage(page.try_into().unwrap()))
    }

    /**
     * Write a block to the device.
     * It's assumed that the data is already aligned to the block size.
     */
    pub async fn write_block(&mut self, offset: u64, data: AlignedPage) -> std::io::Result<()> {
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
        if let Some(fd) = self.fd.take() {
            tokio_uring::spawn(async move {
                fd.close().await.unwrap();
            });
        }
    }
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>>{
    tokio_uring::start(async {
        // Create a new device instance
        let mut device = NvmeDevice::new("/tmp/test1").await.unwrap();

        // Example: Write to offset 0
        let mut write_data = [0u8; BLOCK_SIZE]; // Create a zeroed array
        let hello = b"Hello, world!\n"; // Convert string to bytes
        write_data[..hello.len()].copy_from_slice(hello); // Copy string bytes to start of array
        let write_page = AlignedPage(write_data); // Create aligned page

        // Add this debug print
        println!("Writing bytes: {:?}", &write_page.0[..hello.len()]);

        device.write_block(0, write_page).await.unwrap();
        println!("Write completed");

        // Example: Read from offset 0
        let data = device.read_block(0).await.unwrap();
        println!("Read {} bytes", data.0.len());

        // Log the raw bytes
        // println!("Raw bytes: {:?}", &data[..n]);
        
        println!("Message: {}", String::from_utf8_lossy(&data.0));

        Ok(())
    })
}
