FROM openjdk:11


COPY tools/target/tools-*-jar-with-dependencies.jar .

CMD ["java", "-cp", "tools-*-jar-with-dependencies.jar", "com.automq.elasticstream.client.tools.longrunning.LongRunning"]

