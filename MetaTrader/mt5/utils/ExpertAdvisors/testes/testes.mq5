//+------------------------------------------------------------------+
//|                                                       testes.mq5 |
//|                                  Copyright 2023, MetaQuotes Ltd. |
//|                                             https://www.mql5.com |
//+------------------------------------------------------------------+
//+------------------------------------------------------------------+
//| Expert initialization function                                   |
//+------------------------------------------------------------------+


#include <Zmq/Zmq.mqh>
#include "constant.mqh"
#include "action.mqh"
#include "message.mqh"
#include "operation.mqh"
#include "trade.mqh"

ZmqMsg zmq_req;
uchar zmq_data[];

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
int OnInit()
  {
// Initialize sockets and run in loops
// Get user input data/request
// Process
// Return user info/update
// Repeat

   EventSetMillisecondTimer(MILLISECOND_TIMER);     // Set Millisecond Timer to get client socket input
   context.setBlocky(false);

// Send responses to PULL_PORT that client is listening on.
   if(!pushSocket.bind(StringFormat("%s://%s:%d", ZEROMQ_PROTOCOL, HOSTNAME, PULL_PORT)))
     {
      Print("[PUSH] ####ERROR#### Binding MT4 Server to Socket on Port " + IntegerToString(PULL_PORT) + "..");
      return(INIT_FAILED);
     }
   else
     {
      Print("[PUSH] Binding MT4 Server to Socket on Port " + IntegerToString(PULL_PORT) + "..");
      pushSocket.setSendHighWaterMark(1);
      pushSocket.setLinger(0);
     }

// Receive commands from PUSH_PORT that client is sending to.
   if(!pullSocket.bind(StringFormat("%s://%s:%d", ZEROMQ_PROTOCOL, HOSTNAME, PUSH_PORT)))
     {
      Print("[PULL] ####ERROR#### Binding MT4 Server to Socket on Port " + IntegerToString(PUSH_PORT) + "..");
      return(INIT_FAILED);
     }
   else
     {
      Print("[PULL] Binding MT4 Server to Socket on Port " + IntegerToString(PUSH_PORT) + "..");
      pullSocket.setReceiveHighWaterMark(1);
      pullSocket.setLinger(0);
     }

   return(INIT_SUCCEEDED);
  }
//+------------------------------------------------------------------+
//| Expert deinitialization function                                 |
//+------------------------------------------------------------------+
void OnDeinit(const int reason)
  {

   Print("[PUSH] Unbinding MT4 Server from Socket on Port " + IntegerToString(PULL_PORT) + "..");
   pushSocket.unbind(StringFormat("%s://%s:%d", ZEROMQ_PROTOCOL, HOSTNAME, PULL_PORT));

   Print("[PULL] Unbinding MT4 Server from Socket on Port " + IntegerToString(PUSH_PORT) + "..");
   pullSocket.unbind(StringFormat("%s://%s:%d", ZEROMQ_PROTOCOL, HOSTNAME, PUSH_PORT));

//--- destroy timer
   EventKillTimer();

  }
//+------------------------------------------------------------------+
//| Expert tick function                                             |
//+------------------------------------------------------------------+
void OnTick()
  {
//---

  }
//+------------------------------------------------------------------+
//| Timer function                                                   |
//+------------------------------------------------------------------+
void OnTimer()
  {

// Get client's response, but don't block.
   pullSocket.recv(zmq_req, true);

   if(zmq_req.size() > 0)
     {

      // Wait
      //pullSocket.recv(zmq_request,false);

      // MessageHandler() should go here.
      ZmqMsg reply = MessageHandler(zmq_req);

      // Send response, and block
      //pushSocket.send(reply);

      // Send response, but don't block
      if(!pushSocket.send(reply, true))
        {
         Print("###ERROR### Sending message");
        }
      Print("Sent");
     }


//---

  }

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Trade function                                                   |
//+------------------------------------------------------------------+
void OnTrade()
  {
//---

  }
//+------------------------------------------------------------------+
//| TradeTransaction function                                        |
//+------------------------------------------------------------------+
void OnTradeTransaction(const MqlTradeTransaction& trans,
                        const MqlTradeRequest& request,
                        const MqlTradeResult& result)
  {
//---

  }
//+------------------------------------------------------------------+
//| Tester function                                                  |
//+------------------------------------------------------------------+
double OnTester()
  {
//---
   double ret=0.0;
//---

//---
   return(ret);
  }
//+------------------------------------------------------------------+
//| TesterInit function                                              |
//+------------------------------------------------------------------+
void OnTesterInit()
  {
//---

  }
//+------------------------------------------------------------------+
//| TesterPass function                                              |
//+------------------------------------------------------------------+
void OnTesterPass()
  {
//---

  }
//+------------------------------------------------------------------+
//| TesterDeinit function                                            |
//+------------------------------------------------------------------+
void OnTesterDeinit()
  {
//---

  }
//+------------------------------------------------------------------+
//| ChartEvent function                                              |
//+------------------------------------------------------------------+
void OnChartEvent(const int id,
                  const long &lparam,
                  const double &dparam,
                  const string &sparam)
  {
//---

  }
//+------------------------------------------------------------------+
//| BookEvent function                                               |
//+------------------------------------------------------------------+
void OnBookEvent(const string &symbol)
  {
//---

  }
//+------------------------------------------------------------------+


//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
ZmqMsg MessageHandler(ZmqMsg &request)
  {

// Output object
   ZmqMsg reply;

// Message components for later.
   string components[11];

   if(request.size() > 0)
     {

      // Get data from request
      ArrayResize(zmq_data, (int)request.size());
      request.getData(zmq_data);
      string dataStr = CharArrayToString(zmq_data);

      Print("Received Request: ", dataStr);

      // Process data
      ParseZmqMessage(dataStr, components);

      // Interpret data

      string zmq_ret = InterpretZmqMessage(pushSocket, components) ;

      ZmqMsg reply(StringFormat("%s", zmq_ret));
      pushSocket.send(reply,true); // NON-BLOCKING

      Print("Data being sent out ",zmq_ret);

     }
   else
     {
      // NO DATA RECEIVED
     }

   return(reply);
  }
//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void InformPullClient(Socket& pSocket, string message)
  {


  }
//+------------------------------------------------------------------+
