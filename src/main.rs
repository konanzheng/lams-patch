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

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (600, 600), position: (300, 300), title: "LAMS补丁工具", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    // 工程目录
    #[nwg_control(text: "工程目录", size: (60, 40), position: (10, 10))]
    prj_label:nwg::Label,

    #[nwg_control(text: "E:\\work\\dagl\\bugfix\\Lams", size: (400, 40), position: (70, 10),readonly: true)]
    prj_folder: nwg::TextInput,

    #[nwg_control(text: "选择", size: (100, 40), position: (490, 10))]
    #[nwg_events( OnButtonClick: [BasicApp::choose_prj_folder] )]
    btn_prj_folder: nwg::Button,
    // 发布目录

    #[nwg_control(text: "发布目录", size: (60, 40), position: (10, 60))]
    deploy_label:nwg::Label,

    #[nwg_control(text: "请选择发布目录", size: (400, 40), position: (70, 60),readonly: true)]
    deploy_folder: nwg::TextInput,

    #[nwg_control(text: "选择", size: (100, 40), position: (490, 60))]
    #[nwg_events( OnButtonClick: [BasicApp::choose_deploy_folder] )]
    btn_dep_folder: nwg::Button,

    #[nwg_control(text: "发布ID", size: (60, 40), position: (10, 110))]
    new_id_label:nwg::Label,

    #[nwg_control(size: (510, 40), position: (70, 110))]
    new_id:nwg::ComboBox<String>,
    // #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    // hello_button: nwg::Button,

    #[nwg_control(text: "旧版ID", size: (60, 40), position: (10, 160))]
    old_id_label:nwg::Label,
    #[nwg_control(size: (510, 40), position: (70, 160))]
    old_id:nwg::ComboBox<String>,

    #[nwg_control(text: "刷新历史记录", size: (120, 40), position: (170, 210))]
    #[nwg_events( OnButtonClick: [BasicApp::refresh] )]
    btn_refresh: nwg::Button,

    #[nwg_control(text: "生成补丁", size: (120, 40), position: (310, 210))]
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
        let s = format!("git -C \"{}\" log --pretty=oneline -10", self.prj_folder.text());
        println!("{}", s);
        let output = Command::new(s).output().unwrap();
        let out = String::from_utf8(output.stdout).unwrap();
        println!("{}", out);
        // String cmd = "git -C \"" + bPath.getAbsolutePath() + "\" log --pretty=oneline -10";
        // System.out.println(cmd);
        // List<String> lines = Utils.getExeCmdOutPut(cmd);
           // 执行git 命令 获取commit 历史
            //   let mut cmd = std::process::Command::new("git");
    }
    fn patch(&self) {
        nwg::simple_message("patch", "patch ");
    }

}
fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}