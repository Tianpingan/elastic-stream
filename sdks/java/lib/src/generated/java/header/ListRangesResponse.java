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
public final class ListRangesResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_1_21(); }
  public static ListRangesResponse getRootAsListRangesResponse(ByteBuffer _bb) { return getRootAsListRangesResponse(_bb, new ListRangesResponse()); }
  public static ListRangesResponse getRootAsListRangesResponse(ByteBuffer _bb, ListRangesResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public ListRangesResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public int throttleTimeMs() { int o = __offset(4); return o != 0 ? bb.getInt(o + bb_pos) : 0; }
  public header.ListRangesResult listResponses(int j) { return listResponses(new header.ListRangesResult(), j); }
  public header.ListRangesResult listResponses(header.ListRangesResult obj, int j) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int listResponsesLength() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public header.ListRangesResult.Vector listResponsesVector() { return listResponsesVector(new header.ListRangesResult.Vector()); }
  public header.ListRangesResult.Vector listResponsesVector(header.ListRangesResult.Vector obj) { int o = __offset(6); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }
  public short errorCode() { int o = __offset(8); return o != 0 ? bb.getShort(o + bb_pos) : 0; }
  public String errorMessage() { int o = __offset(10); return o != 0 ? __string(o + bb_pos) : null; }
  public ByteBuffer errorMessageAsByteBuffer() { return __vector_as_bytebuffer(10, 1); }
  public ByteBuffer errorMessageInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 10, 1); }

  public static int createListRangesResponse(FlatBufferBuilder builder,
      int throttleTimeMs,
      int listResponsesOffset,
      short errorCode,
      int errorMessageOffset) {
    builder.startTable(4);
    ListRangesResponse.addErrorMessage(builder, errorMessageOffset);
    ListRangesResponse.addListResponses(builder, listResponsesOffset);
    ListRangesResponse.addThrottleTimeMs(builder, throttleTimeMs);
    ListRangesResponse.addErrorCode(builder, errorCode);
    return ListRangesResponse.endListRangesResponse(builder);
  }

  public static void startListRangesResponse(FlatBufferBuilder builder) { builder.startTable(4); }
  public static void addThrottleTimeMs(FlatBufferBuilder builder, int throttleTimeMs) { builder.addInt(0, throttleTimeMs, 0); }
  public static void addListResponses(FlatBufferBuilder builder, int listResponsesOffset) { builder.addOffset(1, listResponsesOffset, 0); }
  public static int createListResponsesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startListResponsesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
  public static void addErrorCode(FlatBufferBuilder builder, short errorCode) { builder.addShort(2, errorCode, 0); }
  public static void addErrorMessage(FlatBufferBuilder builder, int errorMessageOffset) { builder.addOffset(3, errorMessageOffset, 0); }
  public static int endListRangesResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public ListRangesResponse get(int j) { return get(new ListRangesResponse(), j); }
    public ListRangesResponse get(ListRangesResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public ListRangesResponseT unpack() {
    ListRangesResponseT _o = new ListRangesResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(ListRangesResponseT _o) {
    int _oThrottleTimeMs = throttleTimeMs();
    _o.setThrottleTimeMs(_oThrottleTimeMs);
    header.ListRangesResultT[] _oListResponses = new header.ListRangesResultT[listResponsesLength()];
    for (int _j = 0; _j < listResponsesLength(); ++_j) {_oListResponses[_j] = (listResponses(_j) != null ? listResponses(_j).unpack() : null);}
    _o.setListResponses(_oListResponses);
    short _oErrorCode = errorCode();
    _o.setErrorCode(_oErrorCode);
    String _oErrorMessage = errorMessage();
    _o.setErrorMessage(_oErrorMessage);
  }
  public static int pack(FlatBufferBuilder builder, ListRangesResponseT _o) {
    if (_o == null) return 0;
    int _listResponses = 0;
    if (_o.getListResponses() != null) {
      int[] __listResponses = new int[_o.getListResponses().length];
      int _j = 0;
      for (header.ListRangesResultT _e : _o.getListResponses()) { __listResponses[_j] = header.ListRangesResult.pack(builder, _e); _j++;}
      _listResponses = createListResponsesVector(builder, __listResponses);
    }
    int _errorMessage = _o.getErrorMessage() == null ? 0 : builder.createString(_o.getErrorMessage());
    return createListRangesResponse(
      builder,
      _o.getThrottleTimeMs(),
      _listResponses,
      _o.getErrorCode(),
      _errorMessage);
  }
}

