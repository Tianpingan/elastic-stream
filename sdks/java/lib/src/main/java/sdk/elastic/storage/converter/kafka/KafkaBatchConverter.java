package sdk.elastic.storage.converter.kafka;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.apache.kafka.common.message.LeaderChangeMessage;
import org.apache.kafka.common.message.SnapshotFooterRecord;
import org.apache.kafka.common.message.SnapshotHeaderRecord;
import org.apache.kafka.common.protocol.ByteBufferAccessor;
import org.apache.kafka.common.record.ControlRecordType;
import org.apache.kafka.common.record.ControlRecordUtils;
import org.apache.kafka.common.record.EndTransactionMarker;
import sdk.elastic.storage.models.Record;
import sdk.elastic.storage.models.RecordBatch;
import org.apache.kafka.common.header.Header;
import org.apache.kafka.common.header.internals.RecordHeader;
import org.apache.kafka.common.record.CompressionType;
import org.apache.kafka.common.record.MemoryRecords;
import org.apache.kafka.common.record.MemoryRecordsBuilder;
import org.apache.kafka.common.record.MutableRecordBatch;
import org.apache.kafka.common.record.TimestampType;

/**
 * Kafka batch converter.
 * Note that the record and the batch are equivalent between Kafka and this SDK's model.
 */
public class KafkaBatchConverter {
    private static final String KAFKA_BATCH_ATTRIBUTE_PREFIX = "__KAFKA_";
    private static final String MAGIC_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "m";
    private static final String PARTITION_LEADER_EPOCH_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "pLE";
    private static final String PRODUCER_ID_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "pI";
    private static final String PRODUCER_EPOCH_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "pE";
    private static final String BASE_SEQUENCE_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "bS";
    private static final String IS_CONTROL_BATCH_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "iC";
    private static final String IS_TRANSACTIONAL_BATCH_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "iT";

    private static final String COMPRESSION_TYPE_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "cT";
    private static final String TIMESTAMP_TYPE_IN_PROPERTIES = KAFKA_BATCH_ATTRIBUTE_PREFIX + "tT";
    private static final String KEY_IN_PROPERTIES = "__key";

    /**
     * Convert a Kafka MemoryRecords to a RecordBatch list
     *
     * @param memoryRecords       Kafka MemoryRecords
     * @param streamId            Stream ID
     * @return RecordBatch list
     */
    public static List<RecordBatch> toRecordBatch(MemoryRecords memoryRecords, long streamId) {
        List<RecordBatch> batchList = new ArrayList<>();
        for (MutableRecordBatch batch : memoryRecords.batches()) {
            batchList.add(toRecordBatch(batch, streamId));
        }
        return batchList;
    }

    /**
     * Convert a Kafka RecordBatch to a RecordBatch
     *
     * @param kafkaRecordBatch    Kafka RecordBatch
     * @param streamId            Stream ID
     * @return RecordBatch
     */
    public static RecordBatch toRecordBatch(org.apache.kafka.common.record.RecordBatch kafkaRecordBatch, long streamId) {
        List<Record> recordList = new ArrayList<>();
        Map<String, String> batchAttributes = new HashMap<>();

        batchAttributes.put(MAGIC_IN_PROPERTIES, String.valueOf(kafkaRecordBatch.magic()));
        batchAttributes.put(PARTITION_LEADER_EPOCH_IN_PROPERTIES, String.valueOf(kafkaRecordBatch.partitionLeaderEpoch()));
        batchAttributes.put(PRODUCER_ID_IN_PROPERTIES, String.valueOf(kafkaRecordBatch.producerId()));
        batchAttributes.put(PRODUCER_EPOCH_IN_PROPERTIES, String.valueOf(kafkaRecordBatch.producerEpoch()));
        batchAttributes.put(BASE_SEQUENCE_IN_PROPERTIES, String.valueOf(kafkaRecordBatch.baseSequence()));
        if (kafkaRecordBatch.isControlBatch()) {
            batchAttributes.put(IS_CONTROL_BATCH_IN_PROPERTIES, "");
        }
        if (kafkaRecordBatch.isTransactional()) {
            batchAttributes.put(IS_TRANSACTIONAL_BATCH_IN_PROPERTIES, "");
        }
        batchAttributes.put(COMPRESSION_TYPE_IN_PROPERTIES, String.valueOf(kafkaRecordBatch.compressionType().id));
        batchAttributes.put(TIMESTAMP_TYPE_IN_PROPERTIES, kafkaRecordBatch.timestampType().name);

        for (org.apache.kafka.common.record.Record kafkaRecord : kafkaRecordBatch) {
            Map<String, String> properties = new HashMap<>();
            for (Header header : kafkaRecord.headers()) {
                properties.put(header.key(), new String(header.value(), StandardCharsets.ISO_8859_1));
            }

            // If the record contains a key, store it in properties.
            if (kafkaRecord.keySize() > 0) {
                byte[] keyBytes = new byte[kafkaRecord.keySize()];
                kafkaRecord.key().get(keyBytes);
                properties.put(KEY_IN_PROPERTIES, new String(keyBytes, StandardCharsets.ISO_8859_1));
            }
            // SequenceId may also need to be stored in properties. Ignore it for now. Recover it by baseSequence + index.

            recordList.add(new Record(streamId, kafkaRecord.offset(), kafkaRecord.timestamp(), null, properties, kafkaRecord.value()));
        }

        // add kafkaRecordBatch's attributes to the first record's properties.
        if (!recordList.isEmpty() && !batchAttributes.isEmpty()) {
            recordList.get(0).getProperties().putAll(batchAttributes);
        }

        return new RecordBatch(streamId, recordList);
    }

    /**
     * Convert RecordBatches to a single Kafka MemoryRecords
     *
     * @param recordBatchList       RecordBatch list
     * @return Kafka MemoryRecords
     */
    public static MemoryRecords toKafkaMemoryRecords(List<RecordBatch> recordBatchList) {
        if (recordBatchList == null || recordBatchList.isEmpty()) {
            return MemoryRecords.EMPTY;
        }

        List<MemoryRecords> memoryRecordsList = new ArrayList<>(recordBatchList.size());
        recordBatchList.forEach(recordBatch -> memoryRecordsList.add(toKafkaMemoryRecords(recordBatch)));
        return combineMemoryRecords(memoryRecordsList);
    }

    /**
     * Convert a RecordBatch to a Kafka MemoryRecords.
     * Note that there maybe actually only one batch in the returned MemoryRecords.
     *
     * @param recordBatch           RecordBatch
     * @return Kafka MemoryRecords
     */
    public static MemoryRecords toKafkaMemoryRecords(RecordBatch recordBatch) {
        byte magic = org.apache.kafka.common.record.RecordBatch.MAGIC_VALUE_V2;
        long producerId = -1;
        short producerEpoch = -1;
        int partitionLeaderEpoch = -1;
        int baseSequence = org.apache.kafka.common.record.RecordBatch.NO_SEQUENCE;
        boolean isTransactional = false;
        boolean isControlBatch = false;
        CompressionType compressionType = CompressionType.NONE;
        TimestampType timestampType = TimestampType.CREATE_TIME;

        Map<String, String> baseProperties = recordBatch.getRecords().get(0).getProperties();
        String magicStr = baseProperties.remove(MAGIC_IN_PROPERTIES);
        if (magicStr != null) {
            magic = Byte.parseByte(magicStr);
        }
        String partitionLeaderEpochStr = baseProperties.remove(PARTITION_LEADER_EPOCH_IN_PROPERTIES);
        if (partitionLeaderEpochStr != null) {
            partitionLeaderEpoch = Integer.parseInt(partitionLeaderEpochStr);
        }
        String producerIdStr = baseProperties.remove(PRODUCER_ID_IN_PROPERTIES);
        if (producerIdStr != null) {
            producerId = Long.parseLong(producerIdStr);
        }
        String producerEpochStr = baseProperties.remove(PRODUCER_EPOCH_IN_PROPERTIES);
        if (producerEpochStr != null) {
            producerEpoch = Short.parseShort(producerEpochStr);
        }
        String baseSequenceStr = baseProperties.remove(BASE_SEQUENCE_IN_PROPERTIES);
        if (baseSequenceStr != null) {
            baseSequence = Integer.parseInt(baseSequenceStr);
        }
        isTransactional = baseProperties.remove(IS_TRANSACTIONAL_BATCH_IN_PROPERTIES) != null;
        isControlBatch = baseProperties.remove(IS_CONTROL_BATCH_IN_PROPERTIES) != null;
        String compressionTypeStr = baseProperties.remove(COMPRESSION_TYPE_IN_PROPERTIES);
        if (compressionTypeStr != null) {
            compressionType = CompressionType.forId(Integer.parseInt(compressionTypeStr));
        }
        String timestampTypeStr = baseProperties.remove(TIMESTAMP_TYPE_IN_PROPERTIES);
        if (timestampTypeStr != null) {
            timestampType = TimestampType.forName(timestampTypeStr);
        }
        long firstOffset = recordBatch.getBaseOffset();

        if (isControlBatch) {
            return toKafkaControlMemoryRecords(firstOffset, partitionLeaderEpoch, producerId, producerEpoch, recordBatch);
        }

        ByteBuffer buffer = ByteBuffer.allocate(1024);

        try (MemoryRecordsBuilder builder = MemoryRecords.builder(buffer, magic, compressionType,
            timestampType, firstOffset, org.apache.kafka.common.record.RecordBatch.NO_TIMESTAMP, producerId, producerEpoch, baseSequence,
            isTransactional, isControlBatch, partitionLeaderEpoch)) {
            for (Record record : recordBatch.getRecords()) {
                List<Header> headers = new ArrayList<>();
                ByteBuffer keyByteBuffer = null;
                if (record.getProperties() != null && !record.getProperties().isEmpty()) {
                    String key = record.getProperties().remove(KEY_IN_PROPERTIES);
                    keyByteBuffer = key == null ? null : ByteBuffer.wrap(key.getBytes(StandardCharsets.ISO_8859_1));
                    record.getProperties().forEach((k, v) -> headers.add(new RecordHeader(k, v.getBytes(StandardCharsets.ISO_8859_1))));
                }
                builder.appendWithOffset(record.getOffset(), record.getTimestamp(), keyByteBuffer, record.getBody(), headers.toArray(new Header[0]));
            }
            return builder.build();
        }
    }

    /**
     * Convert a RecordBatch to a Kafka MemoryRecords, which contains only Control Records.
     *
     * @param firstOffset           basic offset of the batch
     * @param partitionLeaderEpoch  partition leader epoch
     * @param producerId            producer id
     * @param producerEpoch         producer epoch
     * @param recordBatch           RecordBatch to be transformed
     * @return Kafka MemoryRecords
     */
    private static MemoryRecords toKafkaControlMemoryRecords(long firstOffset, int partitionLeaderEpoch, long producerId, short producerEpoch, RecordBatch recordBatch) {
        List<MemoryRecords> memoryRecordsList = new ArrayList<>();
        for (Record record : recordBatch.getRecords()) {
            String key = record.getProperties().remove(KEY_IN_PROPERTIES);
            ByteBuffer keyByteBuffer = ByteBuffer.wrap(key.getBytes(StandardCharsets.ISO_8859_1));
            ControlRecordType controlRecordType = ControlRecordType.parse(keyByteBuffer);
            switch (controlRecordType) {
                case ABORT:
                case COMMIT:
                    EndTransactionMarker endTxnMarker = deserializeEndTransactionMarker(controlRecordType, record.getBody());
                    memoryRecordsList.add(MemoryRecords.withEndTransactionMarker(firstOffset, record.getTimestamp(),
                        partitionLeaderEpoch, producerId, producerEpoch, endTxnMarker));
                    break;
                case SNAPSHOT_HEADER:
                    SnapshotHeaderRecord snapshotHeaderRecord = ControlRecordUtils.deserializedSnapshotHeaderRecord(record.getBody());
                    memoryRecordsList.add(MemoryRecords.withSnapshotHeaderRecord(firstOffset, record.getTimestamp(), partitionLeaderEpoch,
                        ByteBuffer.allocate(100), snapshotHeaderRecord));
                    break;
                case SNAPSHOT_FOOTER:
                    SnapshotFooterRecord snapshotFooterRecord = ControlRecordUtils.deserializedSnapshotFooterRecord(record.getBody());
                    memoryRecordsList.add(MemoryRecords.withSnapshotFooterRecord(firstOffset, record.getTimestamp(), partitionLeaderEpoch,
                        ByteBuffer.allocate(100), snapshotFooterRecord));
                    break;
                case LEADER_CHANGE:
                    LeaderChangeMessage leaderChangeMessage = new LeaderChangeMessage(new ByteBufferAccessor(record.getBody()), (short) 0);
                    memoryRecordsList.add(MemoryRecords.withLeaderChangeMessage(firstOffset, record.getTimestamp(), partitionLeaderEpoch,
                        ByteBuffer.allocate(100), leaderChangeMessage));
                    break;
                default:
            }
        }
        return combineMemoryRecords(memoryRecordsList);
    }

    /**
     * Combine multiple memory records into one memory records.
     * If the memory records list is null or empty, return {@link MemoryRecords#EMPTY}.
     * @param memoryRecordsList memory records list
     * @return combined memory records
     */
    private static MemoryRecords combineMemoryRecords(List<MemoryRecords> memoryRecordsList) {
        if (memoryRecordsList == null || memoryRecordsList.isEmpty()) {
            return MemoryRecords.EMPTY;
        }
        if (memoryRecordsList.size() == 1) {
            return memoryRecordsList.get(0);
        }

        int resultBufferSize = 0;
        for (MemoryRecords memoryRecords : memoryRecordsList) {
            resultBufferSize += memoryRecords.sizeInBytes();
        }

        ByteBuffer resultBuffer = ByteBuffer.allocate(resultBufferSize);
        for (MemoryRecords memoryRecords : memoryRecordsList) {
            resultBuffer.put(memoryRecords.buffer());
        }
        resultBuffer.flip();
        return MemoryRecords.readableRecords(resultBuffer);
    }

    /**
     * Deserialize the value of an end transaction marker record.
     * It is based on {@link EndTransactionMarker#deserialize(org.apache.kafka.common.record.Record)}
     *
     * @param type The control record type
     * @param value The value of the record
     * @return The deserialized end transaction marker
     */
    static EndTransactionMarker deserializeEndTransactionMarker(ControlRecordType type, ByteBuffer value) {
        int coordinatorEpoch = value.getInt(2);
        return new EndTransactionMarker(type, coordinatorEpoch);
    }
}
