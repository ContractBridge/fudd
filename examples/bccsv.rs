use ckc_rs::cards::binary_card::{BinaryCard, BC64};
use ckc_rs::cards::HandRanker;
use fudd::types::arrays::five_card::FiveCard;
use fudd::types::arrays::seven_card::SevenCard;
use fudd::types::playing_cards::PlayingCards;

fn main() {
    println!("bc,rank");

    let deck = PlayingCards::deck();
    for (_, b) in deck.combinations(5).enumerate() {
        let five = FiveCard::try_from(b).unwrap().to_five();
        let rank = five.hand_rank_value();
        let bc = BinaryCard::from_five(five);
        println!("{},{}", bc, rank);
    }
    for (_, b) in deck.combinations(7).enumerate() {
        let seven = SevenCard::try_from(b).unwrap().to_seven();
        let rank = seven.hand_rank_value();
        let bc = BinaryCard::from_seven(seven);
        println!("{},{}", bc, rank);
    }
}
