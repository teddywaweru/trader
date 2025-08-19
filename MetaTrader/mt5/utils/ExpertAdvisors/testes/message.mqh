//#import "message"
//int some_stuff(ZmqMsg &parser);
//void ParseZmqMessage(string& message, string& retArray[]);

//#import


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

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void ParseZmqMessage(string& message, string& retArray[])
  {

   string sep = ";";
   ushort u_sep = StringGetCharacter(sep,0);

   int splits = StringSplit(message, u_sep, retArray);

   /*
   for(int i = 0; i < splits; i++) {
      Print(IntegerToString(i) + ") " + retArray[i]);
   }
   */
  }
//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
string InterpretZmqMessage(Socket &pSocket, string &compArray[])
  {
   string scope = compArray[0];
   string request = compArray[1];
   int action = get_action(scope, request);

   if(!CheckOpsStatus(pSocket, action))
      return "{FAILURE}";

   string zmq_ret;

   switch(action)
     {
      case 1:
         Print("Getting Account Info");

         zmq_ret = "{'action': 'GET_ACCOUNT_INFO',";
         GetAccountInfo(zmq_ret);
         zmq_ret += "}";

         break;

      case 2:
         Print("Getting Symbols");
         zmq_ret = "{'action': 'GET_SYMBOLS',";
         GetSymbols(zmq_ret);
         zmq_ret += "}";

         break;

      case 3:
         Print("Getting Symbol Data");

         zmq_ret = "{'action': 'GET_SYMBOL_DATA',";
         GetSymbolData(zmq_ret, compArray[2]);
         zmq_ret += "}";

         break;

	  case 4: 
         Print("Getting Historical Tick Data");

         zmq_ret = "{'action': 'GET_HISTORICAL_DATA',";
         GetSymbolHistData(compArray, zmq_ret);

         zmq_ret += "}";

break;

      case 5:
        {
         Print("Opening New Trade");

         //         "TRADE;OPEN;{:#?};{};{};{};{};{};{:.02};{};{},",
         //self.order_type as u8,
         //self.symbol.name,
         //self.price,
         //self.stop_loss,
         //self.take_profit,
         //self.comment,
         //self.lot_size,
         //self.magic,
         //self.ticket

         MqlTradeRequest request = {};

         request.action = TRADE_ACTION_DEAL;
         request.symbol = compArray[3];
         request.volume = StringToDouble(compArray[8]);
         request.price = StringToDouble(compArray[4]);
         request.sl = StringToDouble(compArray[5]);
         request.tp = StringToDouble(compArray[6]);
         request.deviation = 4;
         request.type = (ENUM_ORDER_TYPE)compArray[2];
         request.comment = compArray[7];
         request.magic = StringToInteger(compArray[8]);
         request.type_filling = ORDER_FILLING_IOC;

         zmq_ret = "{'action': 'NEW_TRADE',";
         OpenNewTrade(request, zmq_ret);
         zmq_ret += "}";
         break;
        }

      case 8:
         Print("Getting Historical Tick Data");

         zmq_ret = "{'action': 'GET_HISTORICAL_DATA',";
         GetSymbolHistData(compArray, zmq_ret);

         zmq_ret += "}";

         break;


      case 10:
         Print("Getting Info for Symbol");
         zmq_ret = "{'action': 'GET_SYMBOL_DATA',";


      default:
         zmq_ret = "{NO DATA}";


     }



   return zmq_ret;
  }


//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void GetAccountInfo(string &zmq_ret)
  {
   zmq_ret += "'account_number':" + (string)AccountInfoInteger(ACCOUNT_LOGIN);
   zmq_ret += ", 'name':'" + AccountInfoString(ACCOUNT_NAME) + "'";
   zmq_ret += ", 'current_time': '" + TimeToString(TimeCurrent()) + "'";
   zmq_ret += ", 'currency': '" + AccountInfoString(ACCOUNT_CURRENCY) + "'";
   zmq_ret += ", 'company': '" + AccountInfoString(ACCOUNT_COMPANY) + "'";
   zmq_ret += ", 'server': '" + AccountInfoString(ACCOUNT_SERVER) + "'";

   ENUM_ACCOUNT_TRADE_MODE account_type = (ENUM_ACCOUNT_TRADE_MODE)AccountInfoInteger(ACCOUNT_TRADE_MODE);
   string trade_mode;
   switch(account_type)
     {
      case ACCOUNT_TRADE_MODE_DEMO:
         trade_mode = "demo";
      case ACCOUNT_TRADE_MODE_CONTEST:
         trade_mode = "contest";
      case ACCOUNT_TRADE_MODE_REAL:
         trade_mode = "real";
      default:
         trade_mode = "NA";
         break;
     }

   zmq_ret += ", 'mode': '" + trade_mode + "'";
   zmq_ret += ", 'current_balance':" + (string)AccountInfoDouble(ACCOUNT_BALANCE);
   zmq_ret += ", 'current_equity':" + (string)AccountInfoDouble(ACCOUNT_EQUITY);
   zmq_ret += ", 'current_profit':" + (string)AccountInfoDouble(ACCOUNT_PROFIT);
   zmq_ret += ", 'free_margin':" + (string)AccountInfoDouble(ACCOUNT_MARGIN_FREE);
   zmq_ret += ", 'leverage' :" + (string)AccountInfoInteger(ACCOUNT_LEVERAGE);

  }
//+------------------------------------------------------------------+
void GetSymbols(string &zmq_ret)
  {
   zmq_ret += "'symbols': [";
   int symbol_total = SymbolsTotal(true);

   for(int i = 0; i < symbol_total - 1; i++)
     {
      zmq_ret += "'"  + SymbolName(i, true) + "'";
      if(i != symbol_total -2)
        {
         zmq_ret += ",";
        }
     }
   zmq_ret += "]";

  }
//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void GetSymbolData(string &zmq_ret, string &symbol)
  {
   zmq_ret += "'symbol_data': {";
   zmq_ret += "'name':'" + symbol + "'";
   zmq_ret += ", 'sector':'" + SymbolInfoString(symbol, SYMBOL_SECTOR_NAME) + "'";
   zmq_ret += ", 'spread':" + (string)SymbolInfoInteger(symbol, SYMBOL_SPREAD);
   zmq_ret += ", 'point':" + (string)SymbolInfoDouble(symbol, SYMBOL_POINT) + "";
   zmq_ret += ", 'bid':" + (string)SymbolInfoDouble(symbol, SYMBOL_BID) + "";
   zmq_ret += ", 'ask':" + (string)SymbolInfoDouble(symbol, SYMBOL_ASK) + "";
   zmq_ret += ", 'tick_value':" + (string)SymbolInfoDouble(symbol, SYMBOL_TRADE_TICK_VALUE) + "";
   zmq_ret += ", 'type_filling': " + (string)SymbolInfoInteger(symbol, SYMBOL_FILLING_MODE) + "";
   zmq_ret += "}";
  }
//+------------------------------------------------------------------+


//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void GetSymbolHistData(string& compArray[], string& zmq_ret)
  {
   Print("Value of compArray[5]", compArray[5]);
   Print("Value of compArray[4]", compArray[4]);
   Print("Value of compArray[3]", compArray[3]);
   Print("Value of compArray[2]", compArray[2]);

   MqlRates rates[];

int timeframe = (int)(StringToInteger(compArray[3]));
//set as hex value;

   //int rates_count = CopyRates(compArray[2], (ENUM_TIMEFRAMES)(compArray[3]), StringToTime(compArray[4]), StringToTime(compArray[5]), rates);
   int rates_count = CopyRates(compArray[2], (ENUM_TIMEFRAMES)(compArray[3]) , StringToInteger(compArray[4]), StringToInteger(compArray[5]), rates);
   if(rates_count < 0)
      return;

   //zmq_ret += "'timeframe': '" + (string)(ENUM_TIMEFRAMES)compArray[3] + "',";
   zmq_ret += "'timeframe': '" + "D1',";

   zmq_ret += "'ticks': [";

   for(int i=0;i<rates_count;i++)
     {
      zmq_ret += "{'time':'" + TimeToString(rates[i].time) + "',";
      zmq_ret += "'open':" + DoubleToString(rates[i].open) + ",";
      zmq_ret += "'high':" + DoubleToString(rates[i].high) + ",";
      zmq_ret += "'low':" + DoubleToString(rates[i].low) + ",";
      zmq_ret += "'close':" + DoubleToString(rates[i].close) + ",";
      zmq_ret += "'tick_volume':" + DoubleToString(rates[i].tick_volume) + "}";
      if(i != rates_count -1)
         zmq_ret += ",";

     }
   zmq_ret += "]";

   Print("ZMQ ret: ", zmq_ret);

  }
//+------------------------------------------------------------------+
