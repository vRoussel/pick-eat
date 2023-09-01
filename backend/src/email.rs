use log::*;
use reqwest;
use serde::Serialize;

const SMTP2GO_API_ROOT: &str = "https://api.smtp2go.com/v3";

#[derive(Serialize)]
struct EmailReq {
    api_key: String,
    to: Vec<String>,
    sender: String,
    subject: String,
    text_body: String,
}

impl std::fmt::Debug for EmailReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EmailReq {{ api_key: <hidden>, to: {:?}, sender: {}, subject: {}, text_body: {} }}",
            self.to, self.sender, self.subject, self.text_body
        )
    }
}

#[derive(Clone)]
pub struct EmailSender {
    api_key: String,
}

impl EmailSender {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn send_account_validation_email(
        self: &Self,
        address: &str,
        token: &str,
        base_url: &str,
    ) -> Result<(), String> {
        let body = EmailReq {
            api_key: self.api_key.clone(),
            to: vec![address.to_owned()],
            sender: "Pick Eat <noreply@pick-eat.fr>".to_owned(),
            subject: "Validation de votre compte pick-eat".to_owned(),
            text_body: format!(
                "Bonjour,\n\
                \n\
                Bienvenue sur Pick Eat ! :)\n\
                Cliquez sur le lien suivant pour valider la création de votre compte :\n\
                {}/account_validation?token={}\n\
                \n\
                A très vite !",
                base_url, token
            ),
        };

        let endpoint = format!("{}/email/send", SMTP2GO_API_ROOT);
        debug!("{:?}", body);
        let client = reqwest::Client::new();
        client
            .post(endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn send_password_reset_email(
        self: &Self,
        address: &str,
        token: &str,
        base_url: &str,
    ) -> Result<(), String> {
        let body = EmailReq {
            api_key: self.api_key.clone(),
            to: vec![address.to_owned()],
            sender: "Pick Eat <noreply@pick-eat.fr>".to_owned(),
            subject: "Mot de passe oublié".to_owned(),
            text_body: format!(
                "Bonjour,\n\
                \n\
                Suite à votre demande, voici un lien pour réinitialiser votre mot de passe :\n\
                {}/password_reset?token={}\n\
                \n\
                Si vous n'êtes pas à l'origine de cette demande, vous pouvez ignorer cet email.
                \n\
                A très vite !",
                base_url, token
            ),
        };

        let endpoint = format!("{}/email/send", SMTP2GO_API_ROOT);
        debug!("{:?}", body);
        let client = reqwest::Client::new();
        client
            .post(endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
