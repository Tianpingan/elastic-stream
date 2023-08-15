FROM openjdk:11

COPY tools/target/tools-1.0-SNAPSHOT-jar-with-dependencies.jar .

CMD ["java", "-cp", "tools-1.0-SNAPSHOT-jar-with-dependencies.jar", "com.automq.elasticstream.client.tools.longrunning.LongRunning"]
