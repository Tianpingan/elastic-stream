import java.nio.ByteBuffer;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
public class ClientJNI {
    private static native void asyncComputation(byte[] input, CompletableFuture<byte[]> future);
    static {
        System.loadLibrary("client");
    }
    public static void main(String[] args) {
        CompletableFuture<byte[]> future = ClientJNI.AsyncFunc("abcd".getBytes());
        try {
            byte[] result = future.get();
            for (int i = 0; i < result.length; i++) {
                System.out.println("result[" + i + "]: " + result[i]);
            }
        } catch (InterruptedException | ExecutionException e) {
            e.printStackTrace();
        }
    } 

    public static CompletableFuture<byte[]> AsyncFunc(byte[] input) {
        // Create a future
        CompletableFuture<byte[]> future = new CompletableFuture<>();
        // Pass the future to jni-function
        ClientJNI.asyncComputation(input, future);
        return future;
    }
}
