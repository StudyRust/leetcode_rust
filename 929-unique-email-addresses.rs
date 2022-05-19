// def num_unique_emails(emails)
//     emails.map do |e|
//         ear = e.split("@")
//         ear[0].split("+")[0].gsub(".", "") + "@" + ear[1]
//     end.uniq.count
// end

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut mms: HashSet<String> = HashSet::new();
    for email in emails {
        let strs: Vec<&str> = email.split("@").collect();
        let ss: Vec<&str> = strs[0].split("+").collect();
        let mut tmp = ss[0].replace(".", "");
        tmp.push_str("@");
        tmp.push_str(strs[1]);
        mms.insert(tmp);
    }
    mms.len() as i32
}

fn main() {
    let emails: Vec<String> = vec!["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]
        .iter().map(|e|e.to_string()).collect();
    println!("{:?}", num_unique_emails(emails));
}