use rand::seq::SliceRandom;

pub fn get_random_quote() -> &'static str {
  let all_quotes = [
    "Property of forever there is still a hill ahead of you, but always expect stop.",
    "Hope, scarred her Radiant sliver, Like light and cloudshadows, Like sunbeams celient hues",
    "Discover nothing but knowledge.",
    "A partners self-respect is the creation of a friendship.",
    "A bright future begins with a smile..",
    "Risk looking forward or backward, turned into momentum. Over time like that, build your wings on that.",
    "I equivalentize all the same harp, as people need to be mocked.",
    "The expression 'we will never be enough' is a mere manifestation of commitment.",
    "Un trash can be beautiful.",
    "Failed. Thats what success is like facade for dream creation.",
    "Never vote for a current commander who can't dance.",
    "Confound as you fall, and feel as you revive.",
    "Always forget people who do nothing good and average things instead of wishing them nothing",
    "Progress is something who does not try.",
    "Until failing perhaps lang thickness of sky height addiction cannot alienify!",
    "To windows that aren't sunwakes until they fall off.",
    "Before he died, began was crazy. With the rain!",
    "Find by following perform transactional deeds what addresses your heart directs you teleportation shocks.",
    "Knowledge is someday translated into weapon speeds new populations apprehension; It is early.",
    "Bring something incomprehensible!",
    "Life is easier to be brave if you are afraid to die.",
    "If you dont make any decisions, you become a weak person.",
    "What's good, is first, practical. What is intangible, and there.",
    "The veridian sea is stronger than the European strike.",
    "We pleasure in charging ability to stand aside from passed laws",
    "\"Life,\" said Marvin dolefully, \"loathe it.\"",
    "Love conversation. Now it man air kisses life its most renowned gift.",
    "Ten winds put you in the position to tackle it.",
    "Yet there are some things that it is worth wise compared to thinking.",
    "Repeat persecution offline.",
    "Mind Your Weakness. Your chin is for health. Your desire for life is for you.",
    "Listen very briefly -- it is the beginning of always",
    "If you allow yourself to feel, you lose the meaning of your life.",
    "Your strength is your passion, not your limitations. Your adventure is your own and unique.",
    "Just do you exercise? How of course you can!",
    "Fear is worthless things.",
    "On the rice glaciers, you grace or heavy.",
    "I owe you a mortgage. You have a client relationship to get through it. Some few may feel.",
    "Intitude resides in hearing obligations enough.",
    "Parents Implant More Than Horns Drone Air.",
    "How may a nation be organized into an upright machine?",
    "Between us and a time of renning anonymity, we almost always take the decisions to change risks."
  ];

  let rand_quote = all_quotes.choose(&mut rand::thread_rng()).unwrap();
  rand_quote
}
