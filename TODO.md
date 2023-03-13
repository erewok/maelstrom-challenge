# To Do

## Broadcast multi-node

The node must somehow keep track (whither raft?) of the accepted values from other nodes.

### Sample

```json
{"src":"c1","dest":"n0","body":{"type":"init","msg_id":1,"node_id":"n0","node_ids":["n1","n2","n3"]}}
{"id":14,"src":"c5","dest":"n0","body":{"type":"topology","topology":{"n0":["n3","n1"],"n1":["n4","n2","n0"],"n2":["n1"],"n3":["n0","n4"],"n4":["n1","n3"]},"msg_id":1}}
{"src":"n3","dest":"n0","body":{"type":"broadcast","msg_id":null,"message":10}}
{"src":"n1","dest":"n0","body":{"type":"broadcast","msg_id":null,"message":20}}
{"id":24,"src":"n3","dest":"n0","body":{"type":"broadcast_ok","in_reply_to":3,"msg_id":3}}
{"id":25,"src":"n1","dest":"n0","body":{"type":"broadcast_ok","in_reply_to":6,"msg_id":3}}

{"id":20,"src":"c10","dest":"n0","body":{"type":"broadcast","message":0,"msg_id":1}}
{"id":24,"src":"n3","dest":"n0","body":{"type":"broadcast_ok","in_reply_to":null,"msg_id":3}}
{"id":25,"src":"n1","dest":"n0","body":{"type":"broadcast_ok","in_reply_to":null,"msg_id":3}}
```

Read papers on Gossip systems and think about design.