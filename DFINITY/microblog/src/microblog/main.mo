import List "mo:base/List";
import Iter "mo:base/Iter";
import Principal "mo:base/Principal";
import Time "mo:base/Time";

actor {
    
    public type Message = {
        message: Text;
        time: Time.Time;
    };
    public type Microblog = actor {
        follow: shared(Principal) -> async ();
        follows: shared query () -> async [Principal];
        post: shared (Text) -> async ();
        // posts: shared query () -> async [Message];
        // timeline: shared () -> async [Message];
        posts: shared query (since: Time.Time) -> async [Message];
        timeline: shared (since: Time.Time) -> async [Message];
    };

    stable var followed : List.List<Principal> = List.nil();

    public shared func follow(id: Principal): async () {
        followed := List.push(id, followed);
    };

    public shared func follows(): async [Principal] {
        List.toArray(followed);
    };

    stable var messages : List.List<Message> = List.nil();

    public shared (msg) func post(text: Text): async() {
        //assert(Principal.toText(msg.caller) == "ec3jx-2jyhs-kmzkn-qu4xr-bo2xq-q3p66-l3iyc-iwxhf-pnlug-ainzk-dae");
        var msg = {
            message = text;
            time = Time.now();
        };  
        messages := List.push<Message>(msg, messages);
    };

    public shared query func posts(since: Time.Time): async [Message] {
        var msgList : List.List<Message> = List.nil();
        for (msg in Iter.fromList<Message>(messages)) {
                if (msg.time >= since) {
                    msgList := List.push<Message>(msg, msgList);
                };
            };
        List.toArray(msgList);
    };

    public shared func timeline(since: Time.Time): async [Message] {
        var all : List.List<Message> = List.nil();

        for (id in Iter.fromList(followed)) {
            let canister : Microblog = actor(Principal.toText(id));
            let msgs = await canister.posts(since);
            for (msg in Iter.fromArray<Message>(msgs)) {
                all := List.push<Message>(msg, all);
            };
        };

        List.toArray(all);
    };



    // public shared query func posts(): async [Message] {
    //     List.toArray(messages);
    // };

    // public shared func timeline(): async [Message] {
    //     var all : List.List<Message> = List.nil();

    //     for (id in Iter.fromList(followed)) {
    //         let canister : Microblog = actor(Principal.toText(id));
    //         let msgs = await canister.posts();
    //         for (msg in Iter.fromArray(msgs)) {
    //             all := List.push(msg, all);
    //         };
    //     };

    //     List.toArray(all);
    // };

};
