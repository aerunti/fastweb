
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport}; 
use  dotenv_codegen::dotenv;

pub fn send_email(to: String, password: String)-> String{
    let email = Message::builder() 
    .from(dotenv!("EMAIL_DEFAULT_FROM").parse().unwrap()) 
    .to(to.parse().unwrap()) 
    .subject("Your new password") 
    .body(String::from(format!("This is your new password: {}",&password)) )
    .unwrap(); 
    println!("{}",dotenv!("EMAIL_SMTP_USERNAME").to_string());
    println!("{}",dotenv!("EMAIL_SMTP_PASSWORD").to_string());
    let creds = Credentials::new(dotenv!("EMAIL_SMTP_USERNAME").to_string(), dotenv!("EMAIL_SMTP_PASSWORD").to_string()); 

    // Open a remote connection to gmail 
    let mailer = SmtpTransport::starttls_relay(dotenv!("EMAIL_SMTP_HOST")) 
        .unwrap() 
        .credentials(creds) 
        .build(); 

        // Send the email 
        let send = match mailer.send(&email) { 
        Ok(_) => "Email sent successfully!".to_string(), 
        Err(e) => format!("Could not send email: {:?}", e), 
    };
    send

}