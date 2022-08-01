fn main() {
    let invitation_1 = Invitation::new("Dolores".to_string(), true, None);
    let invitation_2 = Invitation::new("Cosmo".to_string(), true, Some(String::from("Sorry")));
    println!("{:?}", filter_2(vec![invitation_1, invitation_2]));
}

#[derive(Debug)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: Option<String>
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Invitation {
        Invitation{
            invitee,
            attending,
            message
        }
    }
}

fn filter_1(invitations: Vec<Invitation>)-> Vec<String>{
    let mut result= vec![];
    println!("{:?}", invitations);

    for invitation in invitations {
        if !invitation.attending && invitation.message.is_none(){
            result.push(invitation.invitee.clone());
        }
    }

    result
}

fn filter_2(invitations: Vec<Invitation>) -> Vec<String>{
    let mut result= vec![];

    for invitation in invitations {
        if invitation.attending && invitation.message.is_some(){
            result.push(invitation.message.unwrap());
        }
    }

    result

}

