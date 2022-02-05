# Function
This code watches a folder for changes, e-mails the change (via AWS SES) and removes the file.
Note that at startup, the complete folder is scanned and each file is e-mailed and removed.

For safety reasons, the code that removes the files is commented, to avoid deleting all files in a folder by mistake

# Build
cargo build

# Run
target/debug/crash_report --email "your-aws-ses-verified-e-mail" --subject "a subject for the mail" --path "path to watch"