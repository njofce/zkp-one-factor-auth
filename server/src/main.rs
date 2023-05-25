
use dotenv::dotenv;
use shared::{generate_random_in_range, Q, verify};
use tonic::{transport::Server, Request, Response, Status};
use zkp_auth::{RegisterRequest, RegisterResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse, AuthenticationAnswerRequest, AuthenticationAnswerResponse, auth_server::{Auth, AuthServer}};
use std::collections::HashMap;
use num_bigint::{BigInt};
use num_traits::{FromPrimitive};
use std::str::FromStr;
use std::ops::Sub;
use std::sync::{Arc, Mutex};


pub mod zkp_auth {
  tonic::include_proto!("zkp_auth");
}

pub mod shared;

#[derive(Debug, Default)]
struct Shared {
    _users: Mutex<HashMap<String, UserData>>,
    _challenges: Mutex<HashMap<String, UserChallengeData>>,
    _sessions: Mutex<HashMap<String, String>>
}

#[derive(Clone, Debug, Default)]
pub struct UserData {
    y1: String,
    y2: String,
}

#[derive(Clone, Debug, Default)]
pub struct UserChallengeData {
    r1: String,
    r2: String,
    challenge: String,
}

/**
 * Currently using an in-memory database with HashMaps just to demonstrate the functionality, but in production use case this will use a DB (ex. Postgres).
 */
#[derive(Clone, Debug, Default)]
pub struct Db {
    shared: Arc<Shared>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            shared: Arc::new(Shared {
                _users: Mutex::new(HashMap::new()),
                _challenges: Mutex::new(HashMap::new()),
                _sessions: Mutex::new(HashMap::new()),
            })
        }
    }

    pub fn contains_user(&self, email: &String) -> bool {
        let users = self.shared._users.lock().unwrap();
        return users.contains_key(email);
    }

    pub fn get_user(&self, email: String) -> Option<(String, UserData)> {
        let users = self.shared._users.lock().unwrap();
        match users.get(&email) {
            Some(user) => Some((email, user.clone())),
            None => None,
        }
    }

    pub fn add_user(&self, email: String, y1: String, y2: String) {
        let mut users = self.shared._users.lock().unwrap();
        users.insert(email, UserData { y1: y1, y2: y2 });
    }

    pub fn get_challenge_data(&self, auth_id: String) -> Option<(String, UserChallengeData)> {
        let challenges = self.shared._challenges.lock().unwrap();
        match challenges.get(&auth_id) {
            Some(challenge) => Some((auth_id, challenge.clone())),
            None => None,
        }
    }

    pub fn add_challenge_data(&self, auth_id: String, r1: String, r2: String, challenge: String) {
        let mut challenges = self.shared._challenges.lock().unwrap();

        challenges.insert(auth_id, UserChallengeData { r1: r1, r2: r2, challenge: challenge });
    }

    pub fn get_session_data(&self, auth_id: String) -> Option<(String, String)> {
        let sessions = self.shared._sessions.lock().unwrap();
        match sessions.get(&auth_id) {
            Some(session) => Some((auth_id, session.clone())),
            None => None,
        }
    }

    pub fn add_session(&self, auth_id: String, session_id: String) {
        let mut sessions = self.shared._sessions.lock().unwrap();
        sessions.insert(auth_id, session_id);
    }

}

#[derive(Debug, Default)]
pub struct AuthService {
    user_db: Db,
}

#[tonic::async_trait]
impl Auth for AuthService {

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        let r = request.into_inner();

        let user = r.user;

        if self.user_db.contains_user(&user) {
            println!("User with email {} already exists", &user);
        } else {
            self.user_db.add_user(user, r.y1, r.y2);
        }
       
        return Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let r = request.into_inner();
        
        let mut auth_id = String::new();
        let challenge = generate_random_in_range(&BigInt::from_i32(2).unwrap(), &BigInt::from_str(&Q).unwrap().sub(1)); 

        let user = r.user;

        if self.user_db.contains_user(&user) {
            auth_id.push_str(&user);

            self.user_db.add_challenge_data(user, r.r1, r.r2, challenge.to_string())
        } else {
            auth_id.push_str("NotFound");
        }

        
        return Ok(Response::new(zkp_auth::AuthenticationChallengeResponse { 
            auth_id : auth_id,
            c: challenge.to_string()
        }))
    }

    async fn verify_authentication(&self, request: Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let r = request.into_inner();

        let auth_id = r.auth_id;
        let s = r.s;

        let session = self.user_db.get_challenge_data(auth_id.clone()).unwrap();
        let user = self.user_db.get_user(auth_id.clone()).unwrap();
        
        let y1: BigInt = BigInt::from_str(&user.1.clone().y1).unwrap();
        let y2: BigInt = BigInt::from_str(&user.1.clone().y2).unwrap();
        let r1: BigInt = BigInt::from_str(&session.1.clone().r1).unwrap();
        let r2: BigInt = BigInt::from_str(&session.1.clone().r2).unwrap();
        let c: BigInt = BigInt::from_str(&session.1.clone().challenge).unwrap();
        let s: BigInt = BigInt::from_str(&s).unwrap();

        let result = verify(y1, y2, r1, r2, c, s);

        let mut session_id = String::new();
        
        if result {
            let rnd_session_id = generate_random_in_range(&BigInt::from_i32(2).unwrap(), &BigInt::from_str(&Q).unwrap().sub(1)); 
            session_id.push_str(&rnd_session_id.to_string())
        } else {
            session_id.push_str("BAD CREDENTIALS");
        }

        self.user_db.add_session(auth_id, session_id.clone());

        return Ok(Response::new(zkp_auth::AuthenticationAnswerResponse { 
            session_id: session_id,
        }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server_address = std::env::var("SERVER_ADDRESS");

    let address = server_address.unwrap().parse().unwrap();
    let auth_service = AuthService::default();
   
    println!("Auth Server listening on {}", address);

    Server::builder().add_service(AuthServer::new(auth_service))
      .serve(address)
      .await?;
    Ok(())
       
  }
