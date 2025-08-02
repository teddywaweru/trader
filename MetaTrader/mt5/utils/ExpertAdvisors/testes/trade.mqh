//+------------------------------------------------------------------+
//|                                                           testes |
//|                                  Copyright 2023, MetaQuotes Ltd. |
//|                                             https://www.mql5.com |
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+


//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
#include "structs.mqh"
#include "message.mqh"

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void OpenNewTrade( MqlTradeRequest &request, string& zmq_ret)
  {
  
   MqlTradeResult  result={};

   if(request.volume > 5.0)
     {
      zmq_ret = zmq_ret + ", " + "'response': 'LOT_SIZE_ERROR', 'response_value': 'MAX_LOT_SIZE_EXCEEDED'";
      return;
     }

	 if (!OrderSend(request, result)) 
	 {
	 Print("Error Completing request.\n", GetLastError());
	 Print("retcode:", result.retcode, result.deal, result.order);
	 }
	

//   //int tmpRet = OrderSelect(ticket, SELECT_BY_TICKET, MODE_TRADES);
//struct MqlTradeResult
//  {
//   uint     retcode;          // Operation return code
//   ulong    deal;             // Deal ticket, if it is performed
//   ulong    order;            // Order ticket, if it is placed
//   double   volume;           // Deal volume, confirmed by broker
//   double   price;            // Deal price, confirmed by broker
//   double   bid;              // Current Bid price
//   double   ask;              // Current Ask price
//   string   comment;          // Broker comment to operation (by default it is filled by description of trade server return code)
//   uint     request_id;       // Request ID set by the terminal during the dispatch 
//   int      retcode_external; // Return code of an external trading system
//  };

   zmq_ret += " 'ticket': " + (string)request.order + ", " + "'magic': " + (string)request.magic + ", 'symbol':{";
   GetSymbolData(zmq_ret, PositionGetSymbol((uint)request.order) );
   zmq_ret += "}";
   
   zmq_ret += ", 'volume': " + (string)request.volume + ", 'trade_type': "+ (string)request.type;
   zmq_ret += ", 'open_price': " + (string)request.price + ", 'open_time': '"+ (string)request.type_time + "'";
   zmq_ret += ", 'sl': " + (string)request.sl + ", 'tp': " + (string)request.tp + ", 'pnl' : " + (string)request.type_filling + ", 'comment': '" + request.comment + "'";
   zmq_ret += ", 'lot_size': " + (string)request.volume;



  }
//+------------------------------------------------------------------+
