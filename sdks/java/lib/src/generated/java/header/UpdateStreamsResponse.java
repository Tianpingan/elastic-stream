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
public final class UpdateStreamsResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static UpdateStreamsResponse getRootAsUpdateStreamsResponse(ByteBuffer _bb) { return getRootAsUpdateStreamsResponse(_bb, new UpdateStreamsResponse()); }
  public static UpdateStreamsResponse getRootAsUpdateStreamsResponse(ByteBuffer _bb, UpdateStreamsResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public UpdateStreamsResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public int throttleTimeMs() { int o = __offset(4); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  public header.UpdateStreamResult updateResponses(int j) { return updateResponses(new header.UpdateStreamResult(), j); }
  public header.UpdateStreamResult updateResponses(header.UpdateStreamResult obj, int j) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int updateResponsesLength() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public header.UpdateStreamResult.Vector updateResponsesVector() { return updateResponsesVector(new header.UpdateStreamResult.Vector()); }
  public header.UpdateStreamResult.Vector updateResponsesVector(header.UpdateStreamResult.Vector obj) { int o = __offset(6); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }
  public short errorCode() { int o = __offset(8); return o != 0 ? bb.getShort(o + bb_pos) : 0; }
  public String errorMessage() { int o = __offset(10); return o != 0 ? __string(o + bb_pos) : null; }
  public ByteBuffer errorMessageAsByteBuffer() { return __vector_as_bytebuffer(10, 1); }
  public ByteBuffer errorMessageInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 10, 1); }

  public static int createUpdateStreamsResponse(FlatBufferBuilder builder,
      int throttleTimeMs,
      int updateResponsesOffset,
      short errorCode,
      int errorMessageOffset) {
    builder.startTable(4);
    UpdateStreamsResponse.addErrorMessage(builder, errorMessageOffset);
    UpdateStreamsResponse.addUpdateResponses(builder, updateResponsesOffset);
    UpdateStreamsResponse.addThrottleTimeMs(builder, throttleTimeMs);
    UpdateStreamsResponse.addErrorCode(builder, errorCode);
    return UpdateStreamsResponse.endUpdateStreamsResponse(builder);
  }

  public static void startUpdateStreamsResponse(FlatBufferBuilder builder) { builder.startTable(4); }
  public static void addThrottleTimeMs(FlatBufferBuilder builder, int throttleTimeMs) { builder.addInt(0, throttleTimeMs, 0); }
  public static void addUpdateResponses(FlatBufferBuilder builder, int updateResponsesOffset) { builder.addOffset(1, updateResponsesOffset, 0); }
  public static int createUpdateResponsesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startUpdateResponsesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
  public static void addErrorCode(FlatBufferBuilder builder, short errorCode) { builder.addShort(2, errorCode, 0); }
  public static void addErrorMessage(FlatBufferBuilder builder, int errorMessageOffset) { builder.addOffset(3, errorMessageOffset, 0); }
  public static int endUpdateStreamsResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public UpdateStreamsResponse get(int j) { return get(new UpdateStreamsResponse(), j); }
    public UpdateStreamsResponse get(UpdateStreamsResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public UpdateStreamsResponseT unpack() {
    UpdateStreamsResponseT _o = new UpdateStreamsResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(UpdateStreamsResponseT _o) {
    int _oThrottleTimeMs = throttleTimeMs();
    _o.setThrottleTimeMs(_oThrottleTimeMs);
    header.UpdateStreamResultT[] _oUpdateResponses = new header.UpdateStreamResultT[updateResponsesLength()];
    for (int _j = 0; _j < updateResponsesLength(); ++_j) {_oUpdateResponses[_j] = (updateResponses(_j) != null ? updateResponses(_j).unpack() : null);}
    _o.setUpdateResponses(_oUpdateResponses);
    short _oErrorCode = errorCode();
    _o.setErrorCode(_oErrorCode);
    String _oErrorMessage = errorMessage();
    _o.setErrorMessage(_oErrorMessage);
  }
  public static int pack(FlatBufferBuilder builder, UpdateStreamsResponseT _o) {
    if (_o == null) return 0;
    int _updateResponses = 0;
    if (_o.getUpdateResponses() != null) {
      int[] __updateResponses = new int[_o.getUpdateResponses().length];
      int _j = 0;
      for (header.UpdateStreamResultT _e : _o.getUpdateResponses()) { __updateResponses[_j] = header.UpdateStreamResult.pack(builder, _e); _j++;}
      _updateResponses = createUpdateResponsesVector(builder, __updateResponses);
    }
    int _errorMessage = _o.getErrorMessage() == null ? 0 : builder.createString(_o.getErrorMessage());
    return createUpdateStreamsResponse(
      builder,
      _o.getThrottleTimeMs(),
      _updateResponses,
      _o.getErrorCode(),
      _errorMessage);
  }
}

