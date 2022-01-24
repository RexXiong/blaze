use jni::errors::Result as JniResult;
use jni::JNIEnv;
use jni::objects::JByteBuffer;
use jni::objects::JClass;
use jni::objects::JMethodID;
use jni::objects::JObject;
use jni::objects::JStaticMethodID;
use jni::signature::JavaType;
use jni::signature::Primitive;

#[allow(non_snake_case)]
pub struct JavaClasses<'a> {
    pub cJniBridge: JniBridge<'a>,
    pub cJavaNioSeekableByteChannel: JavaNioSeekableByteChannel<'a>,
    pub cHadoopFileSystem: HadoopFileSystem<'a>,
    pub cHadoopPath: HadoopPath<'a>,
    pub cHadoopFileStatus: HadoopFileStatus<'a>,
    pub cHadoopFSDataInputStream: HadoopFSDataInputStream<'a>,
    pub cJavaList: JavaList<'a>,
    pub cScalaIterator: ScalaIterator<'a>,
    pub cScalaTuple2: ScalaTuple2<'a>,
    pub cSparkManagedBuffer: SparkManagedBuffer<'a>,
    pub cSparkBlazeConverters: SparkBlazeConverters<'a>,
}

unsafe impl<'a> Send for JavaClasses<'a> {} // safety: see JavaClasses::init()
unsafe impl<'a> Sync for JavaClasses<'a> {}

// safety:
//   All jclasses and jmethodids are implemented in raw pointers and can be
//   safely initialized to zero (null)
//
static mut JNI_JAVA_CLASSES: [u8; std::mem::size_of::<JavaClasses>()] = {
    [0; std::mem::size_of::<JavaClasses>()]
};

impl JavaClasses<'static> {
    pub fn init(env: &JNIEnv<'static>) -> JniResult<()> {
        let initialized_java_classes = JavaClasses {
            cJniBridge: JniBridge::new(env).unwrap(),
            cJavaNioSeekableByteChannel: JavaNioSeekableByteChannel::new(env).unwrap(),
            cHadoopFileSystem: HadoopFileSystem::new(env).unwrap(),
            cHadoopPath: HadoopPath::new(env).unwrap(),
            cHadoopFileStatus: HadoopFileStatus::new(env).unwrap(),
            cHadoopFSDataInputStream: HadoopFSDataInputStream::new(env).unwrap(),
            cJavaList: JavaList::new(env).unwrap(),
            cScalaIterator: ScalaIterator::new(env).unwrap(),
            cScalaTuple2: ScalaTuple2::new(env).unwrap(),
            cSparkManagedBuffer: SparkManagedBuffer::new(env).unwrap(),
            cSparkBlazeConverters: SparkBlazeConverters::new(env).unwrap(),
        };
        unsafe {
            // safety:
            //  JavaClasses should be initialized once in jni entrypoint thread
            //  no write/read conflicts will happen
            let jni_java_classes = JNI_JAVA_CLASSES.as_mut_ptr() as *mut JavaClasses;
            *jni_java_classes = initialized_java_classes;
        }
        Ok(())
    }

    pub fn get() -> &'static JavaClasses<'static> {
        return unsafe { // safety: see JavaClasses::init()
            let jni_java_classes = JNI_JAVA_CLASSES.as_ptr() as *const JavaClasses;
            &*jni_java_classes
        };
    }
}

pub struct JniBridge<'a> {
    pub class: JClass<'a>,
    pub method_get_hdfs_file_system: JStaticMethodID<'a>,
    pub method_get_hdfs_file_system_ret: JavaType,
    pub method_get_resource: JStaticMethodID<'a>,
    pub method_get_resource_ret: JavaType,
}
impl<'a> JniBridge<'a> {
    pub const SIG_TYPE: &'static str = "org/apache/spark/sql/blaze/JniBridge";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<JniBridge<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(JniBridge {
            class,
            method_get_hdfs_file_system: env.get_static_method_id(class, "getHDFSFileSystem", "()Lorg/apache/hadoop/fs/FileSystem;")?,
            method_get_hdfs_file_system_ret: JavaType::Object(HadoopFileSystem::SIG_TYPE.to_owned()),
            method_get_resource: env.get_static_method_id(class, "getResource", "(Ljava/lang/String;)Ljava/lang/Object;")?,
            method_get_resource_ret: JavaType::Object(HadoopFileSystem::SIG_TYPE.to_owned()),
        })
    }
}

pub struct JavaNioSeekableByteChannel<'a> {
    pub class: JClass<'a>,
    pub method_read: JMethodID<'a>,
    pub method_read_ret: JavaType,
    pub method_position_set: JMethodID<'a>,
    pub method_position_set_ret: JavaType,
    pub method_size: JMethodID<'a>,
    pub method_size_ret: JavaType,
}
impl<'a> JavaNioSeekableByteChannel<'a> {
    pub const SIG_TYPE: &'static str = "java/nio/channels/SeekableByteChannel";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<JavaNioSeekableByteChannel<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(JavaNioSeekableByteChannel {
            class,
            method_read: env.get_method_id(class, "read", "(Ljava/nio/ByteBuffer;)I")?,
            method_read_ret: JavaType::Primitive(Primitive::Int),
            method_position_set: env.get_method_id(class, "position", "(J)Ljava/nio/channels/SeekableByteChannel;")?,
            method_position_set_ret: JavaType::Object(Self::SIG_TYPE.to_owned()),
            method_size: env.get_method_id(class, "size", "()J")?,
            method_size_ret: JavaType::Primitive(Primitive::Long),
        })
    }
}

pub struct HadoopFileSystem<'a> {
    pub class: JClass<'a>,
    pub method_get_file_status: JMethodID<'a>,
    pub method_get_file_status_ret: JavaType,
    pub method_open: JMethodID<'a>,
    pub method_open_ret: JavaType,
}
impl<'a> HadoopFileSystem<'a> {
    pub const SIG_TYPE: &'static str = "org/apache/hadoop/fs/FileSystem";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<HadoopFileSystem<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(HadoopFileSystem {
            class,
            method_get_file_status: env.get_method_id(class, "getFileStatus", "(Lorg/apache/hadoop/fs/Path;)Lorg/apache/hadoop/fs/FileStatus;")?,
            method_get_file_status_ret: JavaType::Object(HadoopFileStatus::SIG_TYPE.to_owned()),
            method_open: env.get_method_id(class, "open", "(Lorg/apache/hadoop/fs/Path;)Lorg/apache/hadoop/fs/FSDataInputStream;")?,
            method_open_ret: JavaType::Object(HadoopFSDataInputStream::SIG_TYPE.to_owned()),
        })
    }
}

pub struct HadoopPath<'a> {
    pub class: JClass<'a>,
    pub ctor: JMethodID<'a>,
}
impl<'a> HadoopPath<'a> {
    pub const SIG_TYPE: &'static str = "org/apache/hadoop/fs/Path";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<HadoopPath<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(HadoopPath {
            class,
            ctor: env.get_method_id(class, "<init>", "(Ljava/lang/String;)V")?,
        })
    }
}

pub struct HadoopFileStatus<'a> {
    pub class: JClass<'a>,
    pub method_get_len: JMethodID<'a>,
    pub method_get_len_ret: JavaType,
}
impl<'a> HadoopFileStatus<'a> {
    pub const SIG_TYPE: &'static str = "org/apache/hadoop/fs/FileStatus";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<HadoopFileStatus<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(HadoopFileStatus {
            class,
            method_get_len: env.get_method_id(class, "getLen", "()J")?,
            method_get_len_ret: JavaType::Primitive(Primitive::Long),
        })
    }
}

pub struct HadoopFSDataInputStream<'a> {
    pub class: JClass<'a>,
    pub method_seek: JMethodID<'a>,
    pub method_seek_ret: JavaType,
    pub method_read: JMethodID<'a>,
    pub method_read_ret: JavaType,
}
impl<'a> HadoopFSDataInputStream<'a> {
    pub const SIG_TYPE: &'static str = "org/apache/hadoop/fs/FSDataInputStream";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<HadoopFSDataInputStream<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(HadoopFSDataInputStream {
            class,
            method_seek: env.get_method_id(class, "seek", "(J)V")?,
            method_seek_ret: JavaType::Primitive(Primitive::Long),
            method_read: env.get_method_id(class, "read", "(Ljava/nio/ByteBuffer;)I")?,
            method_read_ret: JavaType::Primitive(Primitive::Int),
        })
    }
}

pub struct JavaList<'a> {
    pub class: JClass<'a>,
    pub method_size: JMethodID<'a>,
    pub method_size_ret: JavaType,
    pub method_get: JMethodID<'a>,
    pub method_get_ret: JavaType,
}
impl<'a> JavaList<'a> {
    pub const SIG_TYPE: &'static str = "java/util/List";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<JavaList<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(JavaList {
            class,
            method_size: env.get_method_id(class, "size", "()I")?,
            method_size_ret: JavaType::Primitive(Primitive::Int),
            method_get: env.get_method_id(class, "get", "(I)Ljava/lang/Object;")?,
            method_get_ret: JavaType::Object("java/lang/Object".to_owned()),
        })
    }
}

pub struct ScalaIterator<'a> {
    pub class: JClass<'a>,
    pub method_has_next: JMethodID<'a>,
    pub method_has_next_ret: JavaType,
    pub method_next: JMethodID<'a>,
    pub method_next_ret: JavaType,
}
impl<'a> ScalaIterator<'a> {
    pub const SIG_TYPE: &'static str = "scala/collection/Iterator";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<ScalaIterator<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(ScalaIterator {
            class,
            method_has_next: env.get_method_id(class, "hasNext", "()Z")?,
            method_has_next_ret: JavaType::Primitive(Primitive::Boolean),
            method_next: env.get_method_id(class, "next", "()Ljava/lang/Object;")?,
            method_next_ret: JavaType::Object("java/lang/Object".to_owned()),
        })
    }
}

pub struct ScalaTuple2<'a> {
    pub class: JClass<'a>,
    pub method_1: JMethodID<'a>,
    pub method_1_ret: JavaType,
    pub method_2: JMethodID<'a>,
    pub method_2_ret: JavaType,
}
impl<'a> ScalaTuple2<'a> {
    pub const SIG_TYPE: &'static str = "scala/Tuple2";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<ScalaTuple2<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(ScalaTuple2 {
            class,
            method_1: env.get_method_id(class, "_1", "()Ljava/lang/Object;")?,
            method_1_ret: JavaType::Object("java/lang/Object".to_owned()),
            method_2: env.get_method_id(class, "_2", "()Ljava/lang/Object;")?,
            method_2_ret: JavaType::Object("java/lang/Object".to_owned()),
        })
    }
}

pub struct SparkManagedBuffer<'a> {
    pub class: JClass<'a>,
    pub method_nio_byte_buffer: JMethodID<'a>,
    pub method_nio_byte_buffer_ret: JavaType,
}
impl<'a> SparkManagedBuffer<'a> {
    pub const SIG_TYPE: &'static str = "org/apache/spark/network/buffer/ManagedBuffer";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<SparkManagedBuffer<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(SparkManagedBuffer {
            class,
            method_nio_byte_buffer: env.get_method_id(class, "nioByteBuffer", "()Ljava/nio/ByteBuffer;")?,
            method_nio_byte_buffer_ret: JavaType::Object("java/nio/ByteBuffer".to_owned()),
        })
    }
}

pub struct SparkBlazeConverters<'a> {
    pub class: JClass<'a>,
    pub method_read_managed_buffer_to_segment_byte_channels_as_java: JStaticMethodID<'a>,
    pub method_read_managed_buffer_to_segment_byte_channels_as_java_ret: JavaType,
}
impl<'a> SparkBlazeConverters<'a> {
    pub const SIG_TYPE: &'static str = "org/apache/spark/sql/blaze/execution/Converters";

    pub fn new(env: &JNIEnv<'a>) -> JniResult<SparkBlazeConverters<'a>> {
        let class = env.find_class(Self::SIG_TYPE)?;
        Ok(SparkBlazeConverters {
            class,
            method_read_managed_buffer_to_segment_byte_channels_as_java:
                env.get_static_method_id(
                    class,
                    "readManagedBufferToSegmentByteChannelsAsJava",
                    "(Lorg/apache/spark/network/buffer/ManagedBuffer;)Ljava/util/List;",
                )?,
            method_read_managed_buffer_to_segment_byte_channels_as_java_ret:
                JavaType::Object(JavaList::SIG_TYPE.to_owned()),
        })
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_org_apache_spark_sql_blaze_JniBridge_callNative(
    env: JNIEnv,
    _: JClass,
    taskDefinition: JByteBuffer,
    ipcRecordBatchDataConsumer: JObject,
) {
    if let Err(err) = std::panic::catch_unwind(|| {
        crate::blaze::blaze_call_native(&env, taskDefinition, ipcRecordBatchDataConsumer);
    }) {
        env.throw_new("java/lang/RuntimeException", if let Some(msg) = err.downcast_ref::<String>() {
            &msg
        } else if let Some(msg) = err.downcast_ref::<&str>() {
            msg
        } else {
            "Unknown blaze-rs exception"
        }).unwrap();
    }
}
