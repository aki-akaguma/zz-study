mod test1 {
    use zz_study::sum;
    //
    #[test]
    fn test_01() {
        assert_eq!(sum(1, 3), 4);
        assert_eq!(sum(2u8, 4u8), 6u8);
        assert_eq!(sum(3i64, 5i64), 8i64);
    }
}

mod test2 {
    use std::process::Command;
    //
    #[test]
    fn invoke_shell() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
        };

        let hello = output.stdout;
        let hello = String::from_utf8_lossy(&hello).to_string();
        #[cfg(windows)]
        let hello = hello.replace("\r\n", "\n");
        //
        assert_eq!(hello, "hello\n");
    }
    #[test]
    fn shell_cat_file() {
        let output = if cfg!(target_os = "windows") {
            /*
            Command::new("cmd")
                .args(["/C", r#"cat ".\fixtures\text01.txt""#])
                .output()
                .expect("failed to execute process")
            */
            Command::new("powershell")
                .args(["-Command", r#"cat ".\fixtures\text01.txt""#])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("cat \"./fixtures/text01.txt\"")
                .output()
                .expect("failed to execute process")
        };
        //
        let err_out = output.stderr;
        let err_out = String::from_utf8_lossy(&err_out).to_string();
        #[cfg(windows)]
        let err_out = err_out.replace("\r\n", "\n");
        assert_eq!(err_out, "");
        //
        let hello = output.stdout;
        let hello = String::from_utf8_lossy(&hello).to_string();
        #[cfg(windows)]
        let hello = hello.replace("\r\n", "\n");
        //
        assert_eq!(hello, "abcdefg\nABCDEFG\n");
    }
}
