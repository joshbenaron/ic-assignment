import Queue "canister:queue";

actor Assignment {
    public func post(method: Text, url: Text): async () {
        await Queue.add(method, url);
    }
}