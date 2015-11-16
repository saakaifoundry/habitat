//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use setup;
use util::{command, docker};

#[test]
fn upload_a_package_and_then_install_it() {
    setup::gpg_import();
    setup::key_install();
    setup::simple_service();

    let d = docker::repo("test/simple_service");
    let ipaddress = d.ipaddress();

    let mut upload = command::bldr(&["upload", "simple_service", "-u", &format!("http://{}:9632", ipaddress)]).unwrap();
    upload.wait_with_output();
    assert_cmd_exit_code!(upload, [0]);
    assert_regex!(upload.stdout(), r"Upload Bldr Package (.+)");
    assert_regex!(upload.stdout(), r"simple_service: uploading (.+)");
    assert_regex!(upload.stdout(), r"simple_service: complete");
    let mut install = command::bldr(&["install", "simple_service", "-u", &format!("http://{}:9632", ipaddress)]).unwrap();
    install.wait_with_output();
    assert_cmd_exit_code!(install, [0]);
}
