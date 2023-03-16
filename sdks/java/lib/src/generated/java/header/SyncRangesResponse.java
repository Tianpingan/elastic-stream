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
public final class SyncRangesResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static SyncRangesResponse getRootAsSyncRangesResponse(ByteBuffer _bb) { return getRootAsSyncRangesResponse(_bb, new SyncRangesResponse()); }
  public static SyncRangesResponse getRootAsSyncRangesResponse(ByteBuffer _bb, SyncRangesResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public SyncRangesResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public int throttleTimeMs() { int o = __offset(4); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  public header.SyncRangesResult syncResponses(int j) { return syncResponses(new header.SyncRangesResult(), j); }
  public header.SyncRangesResult syncResponses(header.SyncRangesResult obj, int j) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int syncResponsesLength() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public header.SyncRangesResult.Vector syncResponsesVector() { return syncResponsesVector(new header.SyncRangesResult.Vector()); }
  public header.SyncRangesResult.Vector syncResponsesVector(header.SyncRangesResult.Vector obj) { int o = __offset(6); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }
  public short errorCode() { int o = __offset(8); return o != 0 ? bb.getShort(o + bb_pos) : 0; }
  public String errorMessage() { int o = __offset(10); return o != 0 ? __string(o + bb_pos) : null; }
  public ByteBuffer errorMessageAsByteBuffer() { return __vector_as_bytebuffer(10, 1); }
  public ByteBuffer errorMessageInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 10, 1); }

  public static int createSyncRangesResponse(FlatBufferBuilder builder,
      int throttleTimeMs,
      int syncResponsesOffset,
      short errorCode,
      int errorMessageOffset) {
    builder.startTable(4);
    SyncRangesResponse.addErrorMessage(builder, errorMessageOffset);
    SyncRangesResponse.addSyncResponses(builder, syncResponsesOffset);
    SyncRangesResponse.addThrottleTimeMs(builder, throttleTimeMs);
    SyncRangesResponse.addErrorCode(builder, errorCode);
    return SyncRangesResponse.endSyncRangesResponse(builder);
  }

  public static void startSyncRangesResponse(FlatBufferBuilder builder) { builder.startTable(4); }
  public static void addThrottleTimeMs(FlatBufferBuilder builder, int throttleTimeMs) { builder.addInt(0, throttleTimeMs, 0); }
  public static void addSyncResponses(FlatBufferBuilder builder, int syncResponsesOffset) { builder.addOffset(1, syncResponsesOffset, 0); }
  public static int createSyncResponsesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startSyncResponsesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
  public static void addErrorCode(FlatBufferBuilder builder, short errorCode) { builder.addShort(2, errorCode, 0); }
  public static void addErrorMessage(FlatBufferBuilder builder, int errorMessageOffset) { builder.addOffset(3, errorMessageOffset, 0); }
  public static int endSyncRangesResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public SyncRangesResponse get(int j) { return get(new SyncRangesResponse(), j); }
    public SyncRangesResponse get(SyncRangesResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public SyncRangesResponseT unpack() {
    SyncRangesResponseT _o = new SyncRangesResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(SyncRangesResponseT _o) {
    int _oThrottleTimeMs = throttleTimeMs();
    _o.setThrottleTimeMs(_oThrottleTimeMs);
    header.SyncRangesResultT[] _oSyncResponses = new header.SyncRangesResultT[syncResponsesLength()];
    for (int _j = 0; _j < syncResponsesLength(); ++_j) {_oSyncResponses[_j] = (syncResponses(_j) != null ? syncResponses(_j).unpack() : null);}
    _o.setSyncResponses(_oSyncResponses);
    short _oErrorCode = errorCode();
    _o.setErrorCode(_oErrorCode);
    String _oErrorMessage = errorMessage();
    _o.setErrorMessage(_oErrorMessage);
  }
  public static int pack(FlatBufferBuilder builder, SyncRangesResponseT _o) {
    if (_o == null) return 0;
    int _syncResponses = 0;
    if (_o.getSyncResponses() != null) {
      int[] __syncResponses = new int[_o.getSyncResponses().length];
      int _j = 0;
      for (header.SyncRangesResultT _e : _o.getSyncResponses()) { __syncResponses[_j] = header.SyncRangesResult.pack(builder, _e); _j++;}
      _syncResponses = createSyncResponsesVector(builder, __syncResponses);
    }
    int _errorMessage = _o.getErrorMessage() == null ? 0 : builder.createString(_o.getErrorMessage());
    return createSyncRangesResponse(
      builder,
      _o.getThrottleTimeMs(),
      _syncResponses,
      _o.getErrorCode(),
      _errorMessage);
  }
}

