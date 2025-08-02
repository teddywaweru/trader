//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
int get_action(string& scope, string& request)
  {

   int switch_action = 0;


// Message Structures:


// The 12345678 at the end is the ticket ID, for MODIFY and CLOSE.

// 2) Data Requests

// 2.1) RATES|SYMBOL   -> Returns Current Bid/Ask

// 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME

// NOTE: datetime has format: D'2015.01.01 00:00'

   /*
      scope = TRADE or RATES
      If RATES -> request = Symbol

      If TRADE ->
      scope = TRADE
      request = ACTION (e.g. OPEN, MODIFY, CLOSE)
      compArray[2] = TYPE (e.g. OP_BUY, OP_SELL, etc - only used when ACTION=OPEN)

   // ORDER TYPES:
   // https://docs.mql4.com/constants/tradingconstants/orderproperties

   // OP_BUY = 0
   // OP_SELL = 1
   // OP_BUYLIMIT = 2
   // OP_SELLLIMIT = 3
   // OP_BUYSTOP = 4
   // OP_SELLSTOP = 5

   compArray[3] = Symbol (e.g. EURUSD, etc.)
   compArray[4] = Open/Close Price (ignored if ACTION = MODIFY)
   compArray[5] = SL
   compArray[6] = TP
   compArray[7] = Trade Comment
   compArray[8] = Lots
   compArray[9] = Magic Number
   compArray[10] = Ticket Number (MODIFY/CLOSE)
    */


   /* 02-08-2019 10:41 CEST - HEARTBEAT */
   if(scope == "HEARTBEAT")
      switch_action = 0;
//InformPullClient(pSocket, "{'action_type': 'heartbeat', 'response': 'loud and clear!'}");

   /* Process Messages */
   if(scope == "DATA" && request == "GET_ACCOUNT_INFO")
   Print("Triggered at GET_SYMBOL_DATA");
   
      switch_action = 1;

   if(scope == "DATA" && request == "GET_SYMBOLS")
      switch_action = 2;
      
   if(scope == "DATA" && request == "GET_SYMBOL_DATA")
      switch_action = 3;

   if(scope == "TRADE" && request == "OPEN")
      // 1) Trading Message Structure
      // TRADE|ACTION|TYPE |SYMBOL|PRICE|SL|TP|COMMENT          |TICKET
      // Eg.
      // TRADE|OPEN  | 1   |EURUSD|0    |50|50|R-to-MetaTrader4 |12345678

      switch_action = 3;


   if(scope == "TRADE" && request == "MODIFY")
      switch_action = 3;


   if(scope == "TRADE" && request == "CLOSE")
      switch_action = 3;

   if(scope == "TRADE" && request == "CLOSE_PARTIAL")
      switch_action = 4;

   if(scope == "TRADE" && request == "CLOSE_MAGIC")
      switch_action = 5;

   if(scope == "TRADE" && request == "CLOSE_ALL")
      switch_action = 6;

   if(scope == "TRADE" && request == "GET_OPEN_TRADES")
      switch_action = 7;

   if(scope == "DATA" && request == "GET_HISTORICAL_DATA")
      // 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME
      switch_action = 8;



   return switch_action;
  }

//+------------------------------------------------------------------+
