use aligned_vec::AVec;
use io_uring::{IoUring, Probe, Register};
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use tokio::sync::Mutex;

const BLOCK_SIZE: usize = 4096; // Typical block size, adjust as needed

struct NvmeDevice {
    ring: IoUring,
    fd: std::fs::File,
}

impl NvmeDevice {
    pub fn new(device_path: &str) -> std::io::Result<Self> {
        // Open the NVMe device with direct I/O
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_DIRECT)
            .open(device_path)?;

        // Initialize io_uring with a reasonable queue size
        let ring = IoUring::new(256)?;

        Ok(Self { ring, fd })
    }

    // Read a block at the specified offset
    pub async fn read_block(&mut self, offset: u64) -> std::io::Result<Vec<u8>> {
        // Create an aligned vector
        let mut buffer = AVec::<u8>::with_capacity(BLOCK_SIZE, BLOCK_SIZE);
        buffer.resize(BLOCK_SIZE, 0);

        // Prepare read operation
        let read_e = self.ring.read(
            self.fd.as_raw_fd(),
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            offset,
        )?;

        // Submit and wait for completion
        unsafe {
            self.ring.submit_and_wait(1)?;
        }

        // Get completion result
        let cqe = self.ring.completion().next().expect("No completion")?;

        if cqe.result() < 0 {
            return Err(std::io::Error::from_raw_os_error(-cqe.result()));
        }

        // Convert to regular Vec before returning
        Ok(buffer.to_vec())
    }

    // Write a block at the specified offset
    pub async fn write_block(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        assert!(data.len() <= BLOCK_SIZE, "Data exceeds block size");

        // Create an aligned vector
        let mut buffer = AVec::<u8>::with_capacity(BLOCK_SIZE, BLOCK_SIZE);
        buffer.resize(BLOCK_SIZE, 0);
        buffer[..data.len()].copy_from_slice(data);

        // Prepare write operation
        let write_e = self.ring.write(
            self.fd.as_raw_fd(),
            buffer.as_ptr(),
            buffer.len() as u32,
            offset,
        )?;

        // Submit and wait for completion
        unsafe {
            self.ring.submit_and_wait(1)?;
        }

        // Get completion result
        let cqe = self.ring.completion().next().expect("No completion")?;

        if cqe.result() < 0 {
            return Err(std::io::Error::from_raw_os_error(-cqe.result()));
        }

        Ok(())
    }
}

// Wrapper for thread-safe access
struct AsyncNvmeDevice {
    inner: Arc<Mutex<NvmeDevice>>,
}

impl AsyncNvmeDevice {
    pub fn new(device_path: &str) -> std::io::Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(NvmeDevice::new(device_path)?)),
        })
    }

    pub async fn read_block(&self, offset: u64) -> std::io::Result<Vec<u8>> {
        let mut device = self.inner.lock().await;
        device.read_block(offset).await
    }

    pub async fn write_block(&self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        let mut device = self.inner.lock().await;
        device.write_block(offset, data).await
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a new device instance
    let device = AsyncNvmeDevice::new("/dev/nvme0n1")?;

    // Example: Read from offset 0
    let data = device.read_block(0).await?;
    println!("Read {} bytes", data.len());

    // Example: Write to offset 0
    let write_data = vec![0u8; BLOCK_SIZE];
    device.write_block(0, &write_data).await?;
    println!("Write completed");

    Ok(())
}
