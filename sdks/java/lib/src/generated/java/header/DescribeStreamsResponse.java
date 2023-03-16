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
public final class DescribeStreamsResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static DescribeStreamsResponse getRootAsDescribeStreamsResponse(ByteBuffer _bb) { return getRootAsDescribeStreamsResponse(_bb, new DescribeStreamsResponse()); }
  public static DescribeStreamsResponse getRootAsDescribeStreamsResponse(ByteBuffer _bb, DescribeStreamsResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public DescribeStreamsResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public int throttleTimeMs() { int o = __offset(4); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  public header.DescribeStreamResult describeResponses(int j) { return describeResponses(new header.DescribeStreamResult(), j); }
  public header.DescribeStreamResult describeResponses(header.DescribeStreamResult obj, int j) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int describeResponsesLength() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public header.DescribeStreamResult.Vector describeResponsesVector() { return describeResponsesVector(new header.DescribeStreamResult.Vector()); }
  public header.DescribeStreamResult.Vector describeResponsesVector(header.DescribeStreamResult.Vector obj) { int o = __offset(6); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }
  public short errorCode() { int o = __offset(8); return o != 0 ? bb.getShort(o + bb_pos) : 0; }
  public String errorMessage() { int o = __offset(10); return o != 0 ? __string(o + bb_pos) : null; }
  public ByteBuffer errorMessageAsByteBuffer() { return __vector_as_bytebuffer(10, 1); }
  public ByteBuffer errorMessageInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 10, 1); }

  public static int createDescribeStreamsResponse(FlatBufferBuilder builder,
      int throttleTimeMs,
      int describeResponsesOffset,
      short errorCode,
      int errorMessageOffset) {
    builder.startTable(4);
    DescribeStreamsResponse.addErrorMessage(builder, errorMessageOffset);
    DescribeStreamsResponse.addDescribeResponses(builder, describeResponsesOffset);
    DescribeStreamsResponse.addThrottleTimeMs(builder, throttleTimeMs);
    DescribeStreamsResponse.addErrorCode(builder, errorCode);
    return DescribeStreamsResponse.endDescribeStreamsResponse(builder);
  }

  public static void startDescribeStreamsResponse(FlatBufferBuilder builder) { builder.startTable(4); }
  public static void addThrottleTimeMs(FlatBufferBuilder builder, int throttleTimeMs) { builder.addInt(0, throttleTimeMs, 0); }
  public static void addDescribeResponses(FlatBufferBuilder builder, int describeResponsesOffset) { builder.addOffset(1, describeResponsesOffset, 0); }
  public static int createDescribeResponsesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startDescribeResponsesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
  public static void addErrorCode(FlatBufferBuilder builder, short errorCode) { builder.addShort(2, errorCode, 0); }
  public static void addErrorMessage(FlatBufferBuilder builder, int errorMessageOffset) { builder.addOffset(3, errorMessageOffset, 0); }
  public static int endDescribeStreamsResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public DescribeStreamsResponse get(int j) { return get(new DescribeStreamsResponse(), j); }
    public DescribeStreamsResponse get(DescribeStreamsResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public DescribeStreamsResponseT unpack() {
    DescribeStreamsResponseT _o = new DescribeStreamsResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(DescribeStreamsResponseT _o) {
    int _oThrottleTimeMs = throttleTimeMs();
    _o.setThrottleTimeMs(_oThrottleTimeMs);
    header.DescribeStreamResultT[] _oDescribeResponses = new header.DescribeStreamResultT[describeResponsesLength()];
    for (int _j = 0; _j < describeResponsesLength(); ++_j) {_oDescribeResponses[_j] = (describeResponses(_j) != null ? describeResponses(_j).unpack() : null);}
    _o.setDescribeResponses(_oDescribeResponses);
    short _oErrorCode = errorCode();
    _o.setErrorCode(_oErrorCode);
    String _oErrorMessage = errorMessage();
    _o.setErrorMessage(_oErrorMessage);
  }
  public static int pack(FlatBufferBuilder builder, DescribeStreamsResponseT _o) {
    if (_o == null) return 0;
    int _describeResponses = 0;
    if (_o.getDescribeResponses() != null) {
      int[] __describeResponses = new int[_o.getDescribeResponses().length];
      int _j = 0;
      for (header.DescribeStreamResultT _e : _o.getDescribeResponses()) { __describeResponses[_j] = header.DescribeStreamResult.pack(builder, _e); _j++;}
      _describeResponses = createDescribeResponsesVector(builder, __describeResponses);
    }
    int _errorMessage = _o.getErrorMessage() == null ? 0 : builder.createString(_o.getErrorMessage());
    return createDescribeStreamsResponse(
      builder,
      _o.getThrottleTimeMs(),
      _describeResponses,
      _o.getErrorCode(),
      _errorMessage);
  }
}

