// automatically generated by the FlatBuffers compiler, do not modify

package header;

import com.google.flatbuffers.BaseVector;
import com.google.flatbuffers.BooleanVector;
import com.google.flatbuffers.ByteVector;
import com.google.flatbuffers.Constants;
import com.google.flatbuffers.DoubleVector;
import com.google.flatbuffers.FlatBufferBuilder;
import com.google.flatbuffers.FloatVector;
import com.google.flatbuffers.IntVector;
import com.google.flatbuffers.LongVector;
import com.google.flatbuffers.ShortVector;
import com.google.flatbuffers.StringVector;
import com.google.flatbuffers.Struct;
import com.google.flatbuffers.Table;
import com.google.flatbuffers.UnionVector;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

@SuppressWarnings("unused")
public final class DescribeStreamsRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static DescribeStreamsRequest getRootAsDescribeStreamsRequest(ByteBuffer _bb) { return getRootAsDescribeStreamsRequest(_bb, new DescribeStreamsRequest()); }
  public static DescribeStreamsRequest getRootAsDescribeStreamsRequest(ByteBuffer _bb, DescribeStreamsRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public DescribeStreamsRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public int timeoutMs() { int o = __offset(4); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  public long streamIds(int j) { int o = __offset(6); return o != 0 ? bb.getLong(__vector(o) + j * 8) : 0; }
  public int streamIdsLength() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public LongVector streamIdsVector() { return streamIdsVector(new LongVector()); }
  public LongVector streamIdsVector(LongVector obj) { int o = __offset(6); return o != 0 ? obj.__assign(__vector(o), bb) : null; }
  public ByteBuffer streamIdsAsByteBuffer() { return __vector_as_bytebuffer(6, 8); }
  public ByteBuffer streamIdsInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 6, 8); }

  public static int createDescribeStreamsRequest(FlatBufferBuilder builder,
      int timeoutMs,
      int streamIdsOffset) {
    builder.startTable(2);
    DescribeStreamsRequest.addStreamIds(builder, streamIdsOffset);
    DescribeStreamsRequest.addTimeoutMs(builder, timeoutMs);
    return DescribeStreamsRequest.endDescribeStreamsRequest(builder);
  }

  public static void startDescribeStreamsRequest(FlatBufferBuilder builder) { builder.startTable(2); }
  public static void addTimeoutMs(FlatBufferBuilder builder, int timeoutMs) { builder.addInt(0, timeoutMs, 0); }
  public static void addStreamIds(FlatBufferBuilder builder, int streamIdsOffset) { builder.addOffset(1, streamIdsOffset, 0); }
  public static int createStreamIdsVector(FlatBufferBuilder builder, long[] data) { builder.startVector(8, data.length, 8); for (int i = data.length - 1; i >= 0; i--) builder.addLong(data[i]); return builder.endVector(); }
  public static void startStreamIdsVector(FlatBufferBuilder builder, int numElems) { builder.startVector(8, numElems, 8); }
  public static int endDescribeStreamsRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public DescribeStreamsRequest get(int j) { return get(new DescribeStreamsRequest(), j); }
    public DescribeStreamsRequest get(DescribeStreamsRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}

