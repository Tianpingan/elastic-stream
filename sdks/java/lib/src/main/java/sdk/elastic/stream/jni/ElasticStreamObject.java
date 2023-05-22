package sdk.elastic.stream.jni;
public abstract class ElasticStreamObject implements AutoCloseable{
    static {
        System.loadLibrary("front_end_sdk");
    }
    public long ptr;
}