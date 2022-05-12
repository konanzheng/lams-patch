## NWG DEMO 把补丁包程序 用Rust + NWG 重新造一边轮子。


构建启动不带控制台的程序

``` #![windows_subsystem = "windows"] ```

``` cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup" ```