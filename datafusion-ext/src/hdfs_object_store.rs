use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::Read;
use std::sync::Arc;

use async_trait::async_trait;
use datafusion::datasource::object_store::FileMetaStream;
use datafusion::datasource::object_store::ListEntryStream;
use datafusion::datasource::object_store::ObjectReader;
use datafusion::datasource::object_store::ObjectStore;
use datafusion::datasource::object_store::SizedFile;
use datafusion::error::Result;
use futures::AsyncRead;
use jni::errors::Result as JniResult;
use jni::objects::JObject;
use jni::objects::JValue;
use log::debug;

use crate::jni_bridge::JavaClasses;
use crate::jni_bridge_call_method;
use crate::jni_bridge_call_static_method;
use crate::jni_bridge_new_object;

#[derive(Clone)]
pub struct HDFSSingleFileObjectStore;

impl Debug for HDFSSingleFileObjectStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HDFSObjectStore")
    }
}

#[async_trait::async_trait]
impl ObjectStore for HDFSSingleFileObjectStore {
    async fn list_file(&self, _prefix: &str) -> Result<FileMetaStream> {
        unreachable!()
    }

    async fn list_dir(
        &self,
        _prefix: &str,
        _delimiter: Option<String>,
    ) -> Result<ListEntryStream> {
        unreachable!()
    }

    fn file_reader(&self, file: SizedFile) -> Result<Arc<dyn ObjectReader>> {
        debug!("HDFSSingleFileStore.file_reader: {:?}", file);
        let hdfs_input_stream = Arc::new(
            FSDataInputStreamWrapper::try_new(&file.path)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?,
        );
        Ok(Arc::new(HDFSObjectReader {
            file,
            hdfs_input_stream,
        }))
    }
}

#[derive(Clone)]
struct HDFSObjectReader {
    file: SizedFile,
    hdfs_input_stream: Arc<FSDataInputStreamWrapper>,
}

#[async_trait]
impl ObjectReader for HDFSObjectReader {
    async fn chunk_reader(
        &self,
        _start: u64,
        _length: usize,
    ) -> Result<Box<dyn AsyncRead>> {
        unimplemented!()
    }

    fn sync_chunk_reader(
        &self,
        start: u64,
        _: usize,
    ) -> Result<Box<dyn Read + Send + Sync>> {
        self.get_reader(start)
    }

    fn sync_reader(&self) -> Result<Box<dyn Read + Send + Sync>> {
        self.sync_chunk_reader(0, 0)
    }

    fn length(&self) -> u64 {
        self.file.size
    }
}

impl HDFSObjectReader {
    fn get_reader(&self, start: u64) -> Result<Box<dyn Read + Send + Sync>> {
        let reader = HDFSFileReader {
            hdfs_input_stream: self.hdfs_input_stream.clone(),
            pos: start,
        };
        Ok(Box::new(reader))
    }
}

#[derive(Clone)]
struct HDFSFileReader {
    pub hdfs_input_stream: Arc<FSDataInputStreamWrapper>,
    pub pos: u64,
}

impl Read for HDFSFileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        debug!("HDFSFileReader.read: size={}", buf.len());
        Ok(())
            .and_then(|_| {
                let env = JavaClasses::get_thread_jnienv();
                let buf = env.new_direct_byte_buffer(buf)?;
                let read_size = jni_bridge_call_static_method!(
                    env,
                    JniBridge.readFSDataInputStream,
                    JValue::Object(self.hdfs_input_stream.inner),
                    JValue::Object(buf.into()),
                    JValue::Long(self.pos as i64)
                )?
                .i()? as usize;

                debug!("HDFSFileReader.read result: read_size={}", read_size);
                self.pos += read_size as u64;

                JniResult::Ok(read_size)
            })
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

struct FSDataInputStreamWrapper {
    pub inner: JObject<'static>,
}
unsafe impl Send for FSDataInputStreamWrapper {}
unsafe impl Sync for FSDataInputStreamWrapper {}

impl FSDataInputStreamWrapper {
    pub fn try_new(path: &str) -> JniResult<FSDataInputStreamWrapper> {
        let env = JavaClasses::get_thread_jnienv();
        let fs = jni_bridge_call_static_method!(
            env,
            JniBridge.getHDFSFileSystem,
            JValue::Object(env.new_string(path)?.into())
        )?
        .l()?;
        let path = jni_bridge_new_object!(
            env,
            HadoopPath,
            JValue::Object(env.new_string(path)?.into())
        )?;
        let hdfs_input_stream = jni_bridge_call_method!(
            env,
            HadoopFileSystem.open,
            fs,
            JValue::Object(path)
        )?
        .l()?;
        JniResult::Ok(FSDataInputStreamWrapper {
            inner: hdfs_input_stream,
        })
    }
}

impl Drop for FSDataInputStreamWrapper {
    fn drop(&mut self) {
        let env = JavaClasses::get_thread_jnienv();
        jni_bridge_call_method!(env, HadoopFSDataInputStream.close, self.inner)
            .expect("FSDataInputStream.close() exception");
    }
}
