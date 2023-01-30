import Text "mo:base/Text";
import Map "mo:base/HashMap";
import Debug "mo:base/Debug";
import Option "mo:base/Option";

actor GuestBook {
  // declare regex canister API
  type Re = Text;

  type Match = {
    text : Text;
    start : Nat64;
    end : Nat64;
  };

  let Regex = actor "2looq-saaaa-aaaak-qbv7a-cai" : actor {
    precompile : ([Re]) -> async ();
    is_match : query (Re, Text) -> async Bool;
    is_match_batch : query (Re, [Text]) -> async [Bool];
    captures : query (Re, Text) -> async [?Match];
    captures_batch : query (Re, [Text]) -> async [[?Match]];
  };
  // end declaration

  let emailRegex = "^(\\S+)@(\\S+\\.\\S+)$";
  var compiled = false;

  let guests = Map.HashMap<Text, ()>(10, Text.equal, Text.hash);

  public func register(email : Text) : async () {
    // ensure there are no duplicates
    if (Option.isSome(guests.get(email))) {
      Debug.trap("already on the list");
    };

    // precompile regex first time this method is called
    if (not compiled) {
      Debug.print("precompile");
      await Regex.precompile([emailRegex]);
      compiled := true;
    };

    // validate email
    let valid = await Regex.is_match(emailRegex, email);
    if (not valid) {
      Debug.trap("invalid email!");
    };

    // or do something with capture groups
    let captures = await Regex.captures(emailRegex, email);
    Debug.print(debug_show (captures));

    guests.put(email, ());
  };

  public query func contains(email : Text) : async Bool {
    return Option.isSome(guests.get(email));
  };
};
