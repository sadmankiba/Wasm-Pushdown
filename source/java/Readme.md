# Wasm Java Source

## Install

Install `maven`.

```
sudo apt install maven
```

## Build and Run

To build .wasm, `pom.xml` needs to have this configuration-
```xml
<configuration>
    <mainClass>com.example.wasm.Main</mainClass>
    <targetType>WEBASSEMBLY_WASI</targetType>
    <targetDirectory>${project.build.directory}/wasm</targetDirectory>
    <targetFileName>main.wasm</targetFileName>
    <minifying>true</minifying>
</configuration>
```

Then, run maven as
```sh
mvn clean package
```