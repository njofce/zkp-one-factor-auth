use dotenv::dotenv;
use std::{io, str::FromStr};
use login::{get_login_data, compute_s};
use num_bigint::BigInt;
use register::{get_register_data};
use tonic::Response;
use zkp_auth::{RegisterRequest, RegisterResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse, AuthenticationAnswerRequest, AuthenticationAnswerResponse, auth_client::{AuthClient}};


pub mod login;
pub mod register;
pub mod shared;

pub mod zkp_auth {
  tonic::include_proto!("zkp_auth");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("Connecting to server");
    
    let server_address = std::env::var("SERVER_ADDRESS");

    let address = server_address.unwrap();

    let mut client = AuthClient::connect(address).await?;

    println!("Connected to server");

    let mut command = String::new();

    // Infinite loop waiting for user input
    loop {
        
        println!("Press 1 to register or 2 to login");

        command.clear();

        io::stdin()
            .read_line(&mut command)
            .expect("Error");

        println!("command {}", &command);
        let option = command.trim().parse().unwrap();

        match option {
                1 => {
                    // Register
                    let (email, x, y1, y2) = get_register_data();

                    let _register_res: Response<RegisterResponse> = client.register(
                        tonic::Request::new(
                        RegisterRequest {
                            user:String::from(email),
                            y1: y1,
                            y2: y2
                        },
                    )
                    ).await?;

                    println!("Registered successfully")
                }
                2 => {
                    // Login
                    let (email, x, k, r1, r2) = get_login_data();

                    let challenge_res: Response<AuthenticationChallengeResponse> = client.create_authentication_challenge(
                        tonic::Request::new(
                            AuthenticationChallengeRequest {
                                user:String::from(email),
                                r1: r1,
                                r2: r2,
                            },
                        )
                    ).await?;


                    let auth_id = &challenge_res.get_ref().auth_id;

                    if auth_id == "NotExists" {
                        println!("Email does not exist. Try to register first");
                        continue;
                    }
                    
                    let c: BigInt = BigInt::from_str(&challenge_res.get_ref().c).unwrap();

                    let s = compute_s(x, c, k);


                    let verify_auth_res: Response<AuthenticationAnswerResponse> = client.verify_authentication(
                        tonic::Request::new(
                            AuthenticationAnswerRequest {
                                auth_id:String::from(auth_id),
                                s: s.to_string(),
                            },
                        )
                    ).await?;

                    let session_id = &verify_auth_res.get_ref().session_id;

                    if session_id != "BAD CREDENTIALS" {
                        println!("Successfully logged in");
                    }
                    println!("Session id {}", &session_id);
                }
                _ => {
                    println!("Unknwon!"); 
                    break;
                }
        }
    }

    Ok(())
}