use super::*;

pub struct Encoder<'a>{
    pub inner: CommandEncoder,
    pub queue: &'a Queue
}

impl<'a> Encoder<'a> {
    pub fn compute<'b>(&'b mut self) -> ComputePass<'b> {
        self.inner.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        })
    }
    pub fn copy_buffer(self, buffer: &'a Buffer) -> SourceBuffer<'a> {
        SourceBuffer {
            encoder: self,
            buffer,
            offset: 0,
        }
    }
    pub fn submit(self) -> SubmissionIndex {
        self.queue.submit(Some(self.inner.finish()))
    }
}

pub struct Compute<'a> {
    inner: ComputePass<'a>,
    encoder: Encoder<'a>,
}

impl<'a> Compute<'a> {
    
}

pub struct SourceBuffer<'a> {
    encoder: Encoder<'a>,
    buffer: &'a Buffer,
    offset: BufferAddress,
}

impl<'a> SourceBuffer<'a> {
    pub fn offset(mut self, offset: BufferAddress) -> Self {
        self.offset = offset;
        self
    }
    pub fn to_buffer(self, dest: &'a Buffer) -> ToBuffer<'a> {
        ToBuffer {
            source: self,
            destination: dest,
            offset: 0,
        }
    }
}

pub struct ToBuffer<'a> {
    source: SourceBuffer<'a>,
    destination: &'a Buffer,
    offset: BufferAddress,
}

impl<'a> ToBuffer<'a> {
    pub fn size(mut self, size: BufferAddress) -> Encoder<'a> {
        self.source.encoder.inner.copy_buffer_to_buffer(self.source.buffer, self.source.offset, self.destination, self.offset, size);
        self.source.encoder
    }
}


    // pub fn copy_buffer_to_buffer(mut self, source: &Buffer, source_offset: BufferAddress, destination: &Buffer, destination_offset: BufferAddress, copy_size: BufferAddress) -> Self {
    //     self.inner.copy_buffer_to_buffer(source, source_offset, destination, destination_offset, copy_size);
    //     self
    // }