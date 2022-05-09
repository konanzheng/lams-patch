/*
 * @Author: your name
 * @Date: 2022-04-17 12:32:47
 * @LastEditTime: 2022-04-17 12:33:37
 * @LastEditors: your name
 * @Description: 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 * @FilePath: \gui\src\main.rs
 */
/*!
    A very simple application that show your name in a message box.
    See `basic` for the version without the derive macro
*/
#[warn(unused_imports)]
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;
// use std::io::{BufReader, BufRead};
// #[warn(unused_imports)]
use std::process::{self, Command, Stdio};
use std::fs;
use std::path::*;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (600, 600), position: (300, 300), title: "LAMS补丁工具", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_resource(family: "Microsoft YaHei UI Bold", size: 20, weight: 500)]
    font1: nwg::Font,


    // 工程目录
    #[nwg_control(text: "工程目录", size: (60, 40), position: (10, 10), font: Some(&data.font1))]
    prj_label:nwg::Label,
    // TODO 目前是写死的方便测试，后期要改成 请选择工程目录
    #[nwg_control(text: "E:\\work\\dagl\\bugfix\\Lams", size: (400, 40), position: (70, 10),readonly: true, font: Some(&data.font1))]
    prj_folder: nwg::TextInput,

    #[nwg_control(text: "选择", size: (100, 40), position: (490, 10), font: Some(&data.font1))]
    #[nwg_events( OnButtonClick: [BasicApp::choose_prj_folder] )]
    btn_prj_folder: nwg::Button,
    // 发布目录

    #[nwg_control(text: "发布目录", size: (60, 40), position: (10, 60), font: Some(&data.font1))]
    deploy_label:nwg::Label,
    // TODO 目前是写死的方便测试，后期要改成 请选择发布目录
    #[nwg_control(text: "C:\\Users\\ruizhao\\Desktop\\0425", size: (400, 40), position: (70, 60),readonly: true, font: Some(&data.font1))]
    deploy_folder: nwg::TextInput,

    #[nwg_control(text: "选择", size: (100, 40), position: (490, 60), font: Some(&data.font1))]
    #[nwg_events( OnButtonClick: [BasicApp::choose_deploy_folder] )]
    btn_dep_folder: nwg::Button,

    #[nwg_control(text: "发布ID", size: (60, 40), position: (10, 110), font: Some(&data.font1))]
    new_id_label:nwg::Label,

    #[nwg_control(size: (510, 40), position: (70, 110), font: Some(&data.font1))]
    new_id:nwg::ComboBox<String>,
    // #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    // hello_button: nwg::Button,

    #[nwg_control(text: "旧版ID", size: (60, 40), position: (10, 160), font: Some(&data.font1))]
    old_id_label:nwg::Label,
    #[nwg_control(size: (510, 40), position: (70, 160), font: Some(&data.font1))]
    old_id:nwg::ComboBox<String>,

    #[nwg_control(text: "刷新历史记录", size: (120, 40), position: (170, 210), font: Some(&data.font1))]
    #[nwg_events( OnButtonClick: [BasicApp::refresh] )]
    btn_refresh: nwg::Button,

    #[nwg_control(text: "生成补丁", size: (120, 40), position: (310, 210), font: Some(&data.font1))]
    #[nwg_events( OnButtonClick: [BasicApp::patch] )]
    btn_patch: nwg::Button,

    #[nwg_resource(title: "选择目录", action: nwg::FileDialogAction::OpenDirectory,multiselect:false )]
    dialog: nwg::FileDialog,
}
#[warn(unused_must_use)]
impl BasicApp {
    // fn say_hello(&self) {
    //     nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    // }
    fn say_goodbye(&self) {
        // nwg::simple_message("Goodbye", "Goodbye ");
        nwg::stop_thread_dispatch();
    }
    fn choose_prj_folder(&self) {
        if self.dialog.run(Some(&self.window)) {
            self.prj_folder.set_text("请选择工程目录");
            if let Ok(directory) = self.dialog.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.prj_folder.set_text(&dir);
            }
        }
        self.refresh();
    }
    fn choose_deploy_folder(&self) {
        if self.dialog.run(Some(&self.window)) {
            self.deploy_folder.set_text("请选择发布目录");
            if let Ok(directory) = self.dialog.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.deploy_folder.set_text(&dir);
            }
        }
    }
    fn refresh(&self) {
        let output = Command::new("git").current_dir(self.prj_folder.text()).arg("log").arg("--pretty=oneline").arg("-10").output().unwrap();
        let out = String::from_utf8(output.stdout).unwrap();
        println!("{}", out);
        // let mut lines = out.split("/n").collect();
        let mut collect :Vec<String>= Vec::new();
        let mut collect2 :Vec<String>= Vec::new();
        let lines = out.lines();
        for line in lines {
            let mut format = format!("{},{}", &line[..6], &line[41..]);
            let mut format2 = format!("{},{}", &line[..6], &line[41..]);
            collect.push(format);
            collect2.push(format2);
        }
        self.new_id.set_collection(collect);
        self.new_id.set_selection(Some(0));
        let len = collect2.len()-1;
        self.old_id.set_collection(collect2);
        self.old_id.set_selection(Some(len));        
        // String cmd = "git -C \"" + bPath.getAbsolutePath() + "\" log --pretty=oneline -10";
        // System.out.println(cmd);
        // List<String> lines = Utils.getExeCmdOutPut(cmd);
           // 执行git 命令 获取commit 历史
            //   let mut cmd = std::process::Command::new("git");
    }
    fn patch(&self) {
        // 1. 判断是否选择了 commit id 和 发布目录
        if self.prj_folder.text() == "请选择工程目录" {
            nwg::simple_message("提示", "请选择工程目录");
        }
        let mut new_id_str = "".to_string();
        match self.new_id.selection_string() {
            Some(s) => {
                new_id_str = s;
            },
            None => {
                nwg::simple_message("提示", "请选择新版本号");
                return;
            }
        }
        let mut old_id_str = "".to_string();
        match self.old_id.selection_string() {
            Some(s) => {
                old_id_str = s;
            },
            None => {
                nwg::simple_message("提示", "请选择旧版本号");
                return;
            }
        }
        if old_id_str == "" || new_id_str == "" {
            return
        }
        if self.deploy_folder.text() == "请选择发布目录" {
            nwg::simple_message("提示", "请选择发布目录");
        }
        // 2. 利用git 命令生成 差异信息
        let output = Command::new("git").arg("diff").arg("--name-status").arg(&old_id_str[..6]).arg(&new_id_str[..6]).current_dir(self.prj_folder.text()).output().unwrap();
        let out = String::from_utf8(output.stdout).unwrap();
        println!("{}", out);
        let mut delVec :Vec<&str>= Vec::new();
        let mut copyVec :Vec<&str>= Vec::new();
        for line in out.lines(){
            let split:Vec<&str> = line.split("\t").collect();
            let flag = split[0].to_string();
            let path = split[1];
            if !split[1].to_string().starts_with("src"){
                continue;
            }
            if flag.starts_with("D") || flag.starts_with("R") {
                delVec.push(split[1]);
            }
            if flag.starts_with("R") || flag.starts_with("A") ||flag.starts_with("M") {
                copyVec.push(split[split.len()-1]);
            }
        }
        let mut deploy_path = self.deploy_folder.text();
        let mut prj_path = self.prj_folder.text();
        // TODO 目前实现的很不优雅，后续要处理异常，完善提醒
        for del in delVec{
            del_file(&deploy_path,del);
        }

        for copy in copyVec{
            copy_file(&deploy_path,&prj_path,copy);
        }

        nwg::simple_message("完成", "补丁已生成！");
    }

}
// 常量定义
const WEBAPP: &str = "src/main/webapp";
const JAVA: &str = "src/main/java";
const RESOURCES: &str = "src/main/resources";
const WEBINF: &str = "WEB-INF/classes";
const TARGET: &str = "target/classes";
// 路径转换
fn transform_path(src:String )->String {
    let mut dest = "".to_string();
    if src.starts_with(WEBAPP) {
        dest = src.replace(WEBAPP, "");
    } else if src.starts_with(RESOURCES) {
        dest = src.replace(RESOURCES, WEBINF);
    } else if src.starts_with(JAVA) {
        dest = src.replace(JAVA, WEBINF).replace(".java", ".class");
    }
    println!("路径转换前:{},转换后:{}",src,dest);
    return dest;
}
fn copy_file(to: &str, from:&str,file: &str){
    let mut from2 = from.to_string() +"\\"+ file;
    let dFile = transform_path(file.to_string());
    let mut tf = to.to_string()+"\\"+&dFile;
    if file.starts_with(JAVA) {
        from2 = (from.to_string() +"\\"+ &dFile).replace(WEBINF, TARGET);
    }
    // 创建目录
    fs::create_dir_all(Path::new(&tf).parent().unwrap().to_str().unwrap()).unwrap();
    println!("拷贝从{}到{}",from2,tf);
    fs::copy(&from2, &tf).unwrap();
    // TODO 判断如果是java 文件需要 拷贝内部类
    if file.ends_with(".class") {
        let cfp = Path::new(&from2);
        let name = cfp.file_name().unwrap().to_str().unwrap();
        let parent = cfp.parent().unwrap();
        if !parent.is_dir() || !parent.exists() {
            return;
        }
        let classFiles = parent.read_dir().unwrap();
        for classFile in classFiles {
            if let Ok(entry) = classFile {
                let p = entry.path();
                let name2 = p.file_name().unwrap().to_str().unwrap();
                let ext = p.extension().unwrap();
                if ext == "class" && name2.starts_with(name) {
                    tf = tf.replace(name,name2);
                    println!("拷贝 从{},到{}",p.to_str().unwrap(),tf);
                    // fs::copy(cfp.to_str().unwrap(), ).unwrap();
                }
            }
        }

    }

}
// 删除文件
fn del_file(dir: &str,file: &str) {
    let filePath = dir.to_string()+"\\"+&transform_path(file.to_string());
    let path = Path::new(&filePath);
    if path.is_file() {
        // fs::remove_file(path).unwrap();
        println!("删除文件:{}",path.to_str().unwrap());
    } else if path.is_dir() {
        // fs::remove_dir_all(path).unwrap();
        println!("删除目录:{}",path.to_str().unwrap());
    }
    // TODO 判断如果是java 文件需要 删除内部类
    if path.extension().unwrap() == "class" {
        let name = path.file_name().unwrap().to_str().unwrap();
        let parent = path.parent().unwrap();
        if !parent.is_dir() || !parent.exists() {
            return;
        }
        let classFiles = parent.read_dir().unwrap();
        for classFile in classFiles {
            let p = classFile.unwrap().path();
            if p.extension().unwrap() == "class" && p.file_name().unwrap().to_str().unwrap().starts_with(name) {
                // fs::remove_file(p).unwrap();
                println!("删除内部类文件:{}",p.to_str().unwrap());
            }
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}