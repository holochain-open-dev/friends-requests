# friends-requests
A friends request zome using synchronous send/recieve to be able to see your friends' private profiles 

https://hackmd.io/YctWzHxdSFiwT-u4GyZYvQ


## Testing
testing is multi-agent , so make sure you are running a sim2h server on another terminal tab

## TODO
- profile info should exist on the users source chain to begin with.. or bridged to
- The capfunction to return profile data is missing
- make_friend_request(friend_address) function returns null instead of the capability address

getting this to work.. means understanding this from core... and seeing if its not broken in core:
https://github.com/holochain/holochain-rust/blob/v0.0.15-alpha1/app_spec/zomes/blog/code/src/blog.rs
