import Array "mo:base/Array";
import Types "./types";
import Debug "mo:base/Debug";
import Int "mo:base/Int";

actor Queue {
    type Request = Types.Request;
    
    stable var requests: [Request] = [];
    stable let owner

    public shared(msg) func add(method: Text, url: Text): async () {
        let request: Request = { 
            method = method;
            url = url;
        };

        requests := Array.append<Request>(requests, [request]);
    };

    public shared(msg) func get_and_remove(): async [Request] {
        if 
        let requests2 = requests;
        requests := [];

        return requests2;
    };
}