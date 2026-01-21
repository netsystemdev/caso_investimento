 



 use std::io; 

 use std::io::prelude::*; 

 
 // _______ [  Rust programming language ] _________ 


 /*


        Entrada - investimento de dinheiro e escolher as 4 ações de investimento , 

        porém , entrará com valor maior de aporte


        -> OOP 

        -> Interfaces de validacao 
        
 */
 
 

 

 // _______ < ... macro-class ... > __________


 pub mod class_investidor {
                                          


        // { Rust Class : Investidor } 


        #[derive(Debug , Clone)] 

        pub struct Investidor 
        
        {

               // << -- private -- >> 

               SICS : i64 , // Codigo sistema do investidor ,

               nome : String , 

               renda : f64 , 

               idade : i16 ,  // entrada +18 


               


        }



        impl Investidor 
        
        {


                pub fn new(SICS : i64 , nome : String , renda : f64 , 
              
                     idade : i16 ) -> Investidor {


                            Self {SICS , nome , renda , idade  } 


                     } 



                     // ------------ < property > ----------------------


                     pub fn get_sics(&self) -> &i64 {

                            return &self.SICS;
                     }



                     pub fn get_nome(&self) -> &String {

                            return &self.nome ; 
                     }


                     pub fn get_renda(&self) -> &f64 {

                            return &self.renda;
                     }


                     pub fn get_idade(&self) -> &i16 {

                            return &self.idade;
                     }


                


                pub fn tela(&self)
                   
                {

                     println!("\n");

                     println!("\n");


                     println!("

                                    ------------- INFO -------------------

                                    \n CODIGO SICS  : {} , 

                                    \n NOME         : {},

                                    \n RENDA R$     : {:8.2} ,


                                    \n IDADE        :  {}, 


                            " , self.SICS , self.nome.to_uppercase() , self.renda , self.idade);


                }
                     


        }
                



 }  // [ ....... end module .....]



 // ------------------------------------------------------------------------------------


 // ............. < enum de selecao > ......................
 



 pub mod enum_carteiras_investimento {


      // [ ... aqui , informa com as constantes literais enumeradas as carteiras disponiveis ....] 

       
       use std::convert::TryFrom;

        
        #[derive(Debug , Clone , PartialEq , Eq)]

        pub enum CarteirasAcoesInvestidor {


                  CONSERV_RENDA_FIXA = 1 ,

                  CONSERV_RENDA_SAUDAVEL = 2 ,

                  CONSERV_RENDA_MASTER = 3 , 


               // -------------------------------;;


                  ARROJ_MERCADO_INV = 4,

                  BANCO_SEGURO_PGBK = 5,

                  INSV_INTERNACIONAL_ARROJ = 6, 

                  SELECIONE = 0


        }       
         


        // -------------------- formatacao ----------------    

         use std::fmt;


         impl fmt::Display for CarteirasAcoesInvestidor {


               fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {

                         // formatacao em string 


                          let msg = match self {


                              Self::CONSERV_RENDA_FIXA => "CONSERV : ACAO DE RENDA FIXA " ,

                              Self::CONSERV_RENDA_SAUDAVEL => "CONSERV : ACAO RENDA FIXA + TITULO 7.5%", 

                              Self::CONSERV_RENDA_MASTER => "CONSERV : RENDA MASTER CLASSIC" , 



                              Self::ARROJ_MERCADO_INV => "ARROJ MERCADO - A1" ,

                              Self::BANCO_SEGURO_PGBK => "BANCO SEGURO - RENDA PGBL + ACAO 5.6" , 

                              Self::INSV_INTERNACIONAL_ARROJ => "INSV - RENDA INTERNACIONAL + ACAO COMPOSTA ", 

                              Self::SELECIONE => "SELECIONE",



                          };


                     // _______ [ write ] _________ 


                     write!(f , "{}" , msg) 

               }    


         }




         pub const ARRAY_VALORES : &[CarteirasAcoesInvestidor] = &[


                
               CarteirasAcoesInvestidor::CONSERV_RENDA_FIXA,
               CarteirasAcoesInvestidor::CONSERV_RENDA_SAUDAVEL,
               CarteirasAcoesInvestidor::CONSERV_RENDA_MASTER,
               CarteirasAcoesInvestidor::ARROJ_MERCADO_INV,
               CarteirasAcoesInvestidor::BANCO_SEGURO_PGBK,
               CarteirasAcoesInvestidor::INSV_INTERNACIONAL_ARROJ


         ];



         // _________ iteracao sobre o valor() ----------------- > 
 
         pub fn iter() -> &'static [CarteirasAcoesInvestidor]  { 


                 ARRAY_VALORES
                  

         } // informa a constante + <array associativo dos valores literais > 

       

  
         // ___________ < construcotr impl + formatacao do valor + TryFrom > __________ 
       
       

         // __ TryFrom : conversao + Result<Self para i8> 





        

         impl TryFrom<i8> for CarteirasAcoesInvestidor {

              type Error = String;


        

              fn try_from(x : i8) -> Result<Self , Self::Error> {


                     match x {


                           1 => Ok(Self::CONSERV_RENDA_FIXA) ,
                           
                           2 => Ok(Self::CONSERV_RENDA_SAUDAVEL) , 

                           3 => Ok(Self::CONSERV_RENDA_MASTER),


                           4 => Ok(Self::ARROJ_MERCADO_INV),

                           5 => Ok(Self::BANCO_SEGURO_PGBK), 

                           6 => Ok(Self::INSV_INTERNACIONAL_ARROJ), 

                           _ => Err(format!("Overflow.Error  :: Codigo invalido  :  {}" , x)),

                            


                     }





              }






         } // _______ { end enum-carteira } ______ 







       

 }    // ________ [ mod enum investidor ] ___________ 















 pub mod interface_financeiro {    


         // < .... import ... >

          use class_investidor::Investidor; 

          use enum_carteiras_investimento::CarteirasAcoesInvestidor;

          use stacktrace_exceptions::StackTraceException;




          pub trait ICalculoFinanceiro {


                     // 1 Passo : avaliacao de renda 

                     fn get_avaliacao_renda(&self) -> Result<() , StackTraceException>;

                     fn get_classificacao_acao_inv(&self) -> Result<() , StackTraceException>;  // <' _ informativo da acao _>


                     fn get_entrada_acao_investimento(&self) -> Result<f64 , StackTraceException>; // <' entrada do valor ' _>


                     fn get_requisited_by_currency_value(&self) -> Result<() , StackTraceException>; // <' _ se realmente ele colocou de forma correta _ '> 




                     // <' _ estimativa de rentabilidade em meses : max. 24 meses , tanto carteira conservadora ou arroj _ '> 


                     fn get_margem_rentabilidade(&self , meses : i16 ) -> Result<() , StackTraceException>; 





          }


 }
 



 // ____________ << classe de negocio : PerfilInvestidorAcao >> ___________________


 pub mod class_perfil_investidor {


     //  << -------- modulo em que ele comprará a acao e escolha do aporte para compra ---------->>

     // <<  -------- importacao ------->> 


     use enum_carteiras_investimento::CarteirasAcoesInvestidor; 

     use class_investidor::Investidor; 

     use interface_financeiro::ICalculoFinanceiro;

     use stacktrace_exceptions::StackTraceException;





     // < ----------- Classe de negocio -------------> 

     #[derive(Debug , Clone)]


     pub struct PerfilInvestidorAcoes {


                ano_concessao : i16 , 

                investidor    : Investidor ,  

                carteira      : CarteirasAcoesInvestidor, 

                aporte_inv    : f64


     }



     // -------------- < constructor > ----------------------


     impl PerfilInvestidorAcoes {
                                    // & -> emprestimo (ownership)

              pub fn new(ano_concessao : i16 , investidor : &Investidor , 

                     carteira : &CarteirasAcoesInvestidor , aporte_inv : f64) -> PerfilInvestidorAcoes {


                       Self {


                               ano_concessao,
                               
                               investidor : investidor.clone(),

                               carteira   : carteira.clone(),

                               aporte_inv,


                       }     

              }


              // < -------------- property ------------------->


              pub fn get_ano_concessao(&self) -> &i16 {

                      return &self.ano_concessao;
              
              }



              pub fn get_investidor(&self) -> &Investidor {

                      return &self.investidor;
              

              }


              
              pub fn get_carteira(&self) -> &CarteirasAcoesInvestidor {

                       return &self.carteira;

              }      



              // -------------- [[ aporte do investidor ]] -------------


              pub fn get_aporte_inv(&self) -> &f64 {

                       return &self.aporte_inv;
              }



              pub fn tela(&self) {


                      println!("\n");


                      println!("


                                -------------- CATEGORIA - PERFIL DE INVESTIDOR -----------------


                                \n ANO DE CONCESSAO _  {}  , 

                                 \n CARTEIRA SELECIONADA _  {}  ,

                                 \n APORTE INICIAL :  _ {:8.2}   , 
                                   

                            ", self.ano_concessao , self.carteira , self.aporte_inv);
                                 


                      self.investidor.tela();
                                 
                                
               }




           }
 

    



     // --------------- << assinaturaas  + interface  >> --------------------


     impl ICalculoFinanceiro for PerfilInvestidorAcoes {


              // funcoes aqui!

                // 1 Passo : avaliacao de renda 

                fn get_avaliacao_renda(&self) -> Result<() , StackTraceException>

                {

                       /*
                        * 
                        *  1. > _ ter 1 salario minimo , para compra basica de uma acao
                           2. > _ aporte de compra de , no minimo , 500.00 
                        * 
                        */

                        // ---------> efetua a desreferência ------------>

                        let mut _class_renda_investidor : f64 = *self.get_investidor().get_renda();

                        let mut _class_aporte_investidor_compra : f64 = *self.get_aporte_inv();

                       



                        if _class_renda_investidor < 1518.00 || _class_aporte_investidor_compra < 500.00 
                         
                            

                        {
                             
                             return Err(StackTraceException::FAILED_REQUISITED_BY_ACCOUNT) 


                        } 

                     
                        
                        
                     Ok(())   

                }  




                fn get_classificacao_acao_inv(&self) -> Result<() , StackTraceException>
                
                {
                       // ______ Cada ação , deverá informar o valor para dar entrada ___ 

                       let mut _enum_class_carteiras_acoes = self.get_carteira();

                        
                        // Apresenta cada valor da enumeracao os seus valores para aquisicao 

                     

                        let _ = match _enum_class_carteiras_acoes 
                        
                        {

                                CarteirasAcoesInvestidor::CONSERV_RENDA_FIXA => { 

                                     println!("VALOR DE AQUISICAO  : R$ 500,00");
                                   

                                }, 


                                CarteirasAcoesInvestidor::CONSERV_RENDA_SAUDAVEL => {


                                     println!("VALOR DE AQUISICAO  : R$ 700,00 ") ; 
                                
                                } ,


                                CarteirasAcoesInvestidor::CONSERV_RENDA_MASTER => {


                                     println!("VALOR DE AQUISICAO - LINHA DE APORTE : R$ 800,00");
                                },



                                CarteirasAcoesInvestidor::ARROJ_MERCADO_INV => {


                                     println!("APORTE INICIAL :  R$ 1000,00"); 
                                },



                                CarteirasAcoesInvestidor::BANCO_SEGURO_PGBK => {


                                     println!("APORTE + SEGURO DO BANCO  : R$ 2400,00");
                                },


                                CarteirasAcoesInvestidor::INSV_INTERNACIONAL_ARROJ => {

                                     println!("SEG. INTERNACIONAL + 3000,00 + 2.4% ACAO "); 

                                },



                                  // _ para opcao errada _ 

                                  _ => {

                                          return Err(StackTraceException::MEMORY_ARITHMETIC_EXCEPTION)
                                  }



                        };
                        

                         


                     Ok(())
                }



                 // _____ funcao que verifica a entrada do aporte x aprovacao da compra da acao 

                 fn get_entrada_acao_investimento(&self) -> Result<f64 , StackTraceException> 

                 {

                      let mut _enum_class_carteiras_acoes = self.get_carteira();
                     
                      let mut _class_valor_aporte = self.get_aporte_inv(); 



                      // valor de cada acao a ser comprada 
                     
                      let mut valor_total : f64 = 0.00 ; 


                      let _ = match _enum_class_carteiras_acoes {



                                  CarteirasAcoesInvestidor::CONSERV_RENDA_FIXA => { 

                                      
                                          valor_total = 500.00;

                                }, 


                                CarteirasAcoesInvestidor::CONSERV_RENDA_SAUDAVEL => {


                                           valor_total = 700.00;
                                } ,


                                CarteirasAcoesInvestidor::CONSERV_RENDA_MASTER => {


                                            valor_total = 800.00;
                                },



                                CarteirasAcoesInvestidor::ARROJ_MERCADO_INV => {


                                            valor_total = 1000.00; 
                                },   



                                CarteirasAcoesInvestidor::BANCO_SEGURO_PGBK => {

                                            valor_total = 2400.00; 

                                            
                                },


                                CarteirasAcoesInvestidor::INSV_INTERNACIONAL_ARROJ => {

                                     
                                             valor_total = 3102.00 // 
                                },



                                  _ => {


                                          return Err(StackTraceException::MEMORY_ARITHMETIC_EXCEPTION);
                                  }




                      }; // _____ << end selection >> ____ 





                     Ok(valor_total) // < __ retorna o valor _ > 
                 

                 }





                 // _ < validations > _ 

                 fn get_requisited_by_currency_value(&self) -> Result<() , StackTraceException> {


                      // -> enumeracao , responsavel pela selecao da constante


                       let mut _enum_class_carteiras_acoes = self.get_carteira();

                       let mut _validacao_entrada_valor = *self.get_aporte_inv();    // <' _ desreferencia _ '>


                       let _ = match _enum_class_carteiras_acoes {


                              // < classifica a validacao pelo que está sendo digitado > 


                               CarteirasAcoesInvestidor::CONSERV_RENDA_FIXA => {


                                       if _validacao_entrada_valor < 500.00 {

                                             return Err(StackTraceException::FAILED_REQUISITED_BY_ACCOUNT); 
                                       
                                       }

                               },



                               CarteirasAcoesInvestidor::CONSERV_RENDA_SAUDAVEL => {

                                         if _validacao_entrada_valor < 700.00 {

                                            return Err(StackTraceException::FAILED_REQUISITED_BY_ACCOUNT); 
                                       
                                       }


                               }, 



                               CarteirasAcoesInvestidor::CONSERV_RENDA_MASTER => {

                                         if _validacao_entrada_valor < 800.00 {

                                            return Err(StackTraceException::FAILED_REQUISITED_BY_ACCOUNT); 

                                         } 

                               }, 




                               // _____________ <'_ validacao de entrada : acoes arrojadas   '_ > ______________


                               CarteirasAcoesInvestidor::ARROJ_MERCADO_INV => {



                                         if _validacao_entrada_valor < 1000.00 {

                                            return Err(StackTraceException::FAILED_REQUISITED_BY_ACCOUNT); 

                                         } 


                               }, 



                               CarteirasAcoesInvestidor::BANCO_SEGURO_PGBK => {



                                         if _validacao_entrada_valor < 2400.00 {

                                            return Err(StackTraceException::FAILED_REQUISITED_BY_ACCOUNT); 

                                         }   

                               },



                               CarteirasAcoesInvestidor::INSV_INTERNACIONAL_ARROJ => {



                                         if _validacao_entrada_valor < 3102.00 {

                                            return Err(StackTraceException::FAILED_REQUISITED_BY_ACCOUNT); 

                                         }  

                               }, 





                               _ => return Err(StackTraceException::MEMORY_ARITHMETIC_EXCEPTION), 



                       }; // <' _ end-case _ '>



                     Ok(())
                 }





                 // <' _ funcao mestre : projecao de rentabilidade  ex: minino : 6 meses / max 24 meses _ '>


                  fn get_margem_rentabilidade(&self , meses : i16 ) -> Result<() , StackTraceException> 

                  {


                       let mut _enum_class_carteiras_acoes = self.get_carteira();

                       let mut aporte_valor : f64 = *self.get_aporte_inv();    // <' _ desreferencia _ '>
                       


                       // <'_ pre validacao dos meses ' _ >


                       if meses < 6 || meses > 24 {


                                 return Err(StackTraceException::MEMORY_ARITHMETIC_EXCEPTION) 

                       }



                       // _ <' _ caso nao haja erros _ '> 


                       let _  = match _enum_class_carteiras_acoes {


                                
                               CarteirasAcoesInvestidor::CONSERV_RENDA_FIXA => {


                                     let mut constant_margem : f64 = 0.045;   // <'_ margem ficticia de 4.5% / mes '_> 


                                          for i in 1 .. meses {


                                                  println!("MES   -  {}  /  RENTABILIDADE MENSAL  %/m :  {:.2}" , i , aporte_valor * constant_margem);


                                          }



                               }, 





                               CarteirasAcoesInvestidor::CONSERV_RENDA_SAUDAVEL => {

  
                                       let mut constant_margem : f64 = 0.05;   // <'_ margem ficticia de 5.0% / mes '_> 

                             
                                           for i in 1 ..=meses {


                                                  println!("MES   -  {}  /  RENTABILIDADE MENSAL  %/m :  {:.2}" , i , aporte_valor * constant_margem);

                                           }


                               }, 



                               CarteirasAcoesInvestidor::CONSERV_RENDA_MASTER => {



                                       let mut constant_margem : f64 = 0.650;   // <'_ margem ficticia de 6.50% / mes '_> 

                             
                                           for i in 1 ..=meses {


                                                  println!("MES   -  {}  /  RENTABILIDADE MENSAL  %/m :  {:.2}" , i , aporte_valor * constant_margem);

                                           }


                               },






                               CarteirasAcoesInvestidor::ARROJ_MERCADO_INV => {


                                          let mut constant_margem : f64 = 0.1050;   // <'_ margem ficticia de 10.50% / mes '_> 

                             
                                           for i in 1 ..=meses {


                                                  println!("MES   -  {}  /  RENTABILIDADE MENSAL  %/m :  {:.2}" , i , aporte_valor * constant_margem);

                                           }
         


                               }, 




                               CarteirasAcoesInvestidor::BANCO_SEGURO_PGBK => {


                                          let mut constant_margem : f64 = 0.12;   // <'_ margem ficticia de 12.% / mes '_> 

                             
                                           for i in 1 ..=meses {


                                                  println!("MES   -  {}  /  RENTABILIDADE MENSAL  %/m :  {:.2}" , i , aporte_valor * constant_margem);

                                           }
         


                               }, 




                               CarteirasAcoesInvestidor::INSV_INTERNACIONAL_ARROJ => {


                                           let mut constant_margem : f64 = 0.1650;   // <'_ margem ficticia de 12.% / mes '_> 

                             
                                           for i in 1 ..=meses {


                                                  println!("MES   -  {}  /  RENTABILIDADE MENSAL  %/m :  {:.2}" , i , aporte_valor * constant_margem);

                                           }
         





                               },





                                // <' _ fora da enumeracao :: exception de memoria de calculo _'> 


                               _ => return Err(StackTraceException::MEMORY_ARITHMETIC_EXCEPTION),



                       }; //end match _ 







                     Ok(())

                  


                  } // end-function



     }
 


 } 




 
 







 // _____________ << main-class >> _____________________



 type TRuntimeClassException<T> = Result<T , Box<dyn std::error::Error>>;



 fn main() -> TRuntimeClassException<()> {



       // invoca a funcao + result<T> validada 


       let _ = entrada_dados()?;





    io::stdout().flush()?;

    io::stdin().read(&mut [0u8])?;

    Ok(())
 }


 



 // ____________ < importam-se as bases , para uso e encapsulamento das classes > _______________



 use std::process::exit; 

 use enum_carteiras_investimento::*;

 use class_investidor::Investidor;

 use interface_financeiro::ICalculoFinanceiro;

 use stacktrace_exceptions::StackTraceException as exceptions;

 use class_perfil_investidor::PerfilInvestidorAcoes;

 

 fn entrada_dados() ->  TRuntimeClassException<()> {


    // _____ entrada de dados 

    println!("\n");

    println!(" ---------------- ACOES DE MERCADO - INVESTIMENTO -----------------");

    println!("\n");
 

    println!(" <<< --- tecle para continuar ---- >>> "); 

    io::stdin().read(&mut [0u8])?;

    println!("\n");


  // ------------------------------------------------------------------------------------


  print!("INFORME N DO SICS GERADO  _  "); 

  let _campo1 = entry_str_sics()?;

  
  let _ = match is_correct_numeric(&_campo1) {

         Ok(val) => val ,

         Err(string_error) => return Err(string_error)

               
 
  };


  // conversao


  let int_conv_sics : i64 = match int64_parse_convert(&_campo1) {


         Ok(val) => val ,

         Err(parse_exception) => {

                panic!("ParseIntException : Abort Critic Error by failed conversion  :  {} " , parse_exception);

         },


  };



  // -----------------------------------------------------------

 
  print!("INVESTIDOR   _ ") ; 

  let _campo2 = entry_str_nome()?;
  

  let _ = match is_correct_letters(&_campo2) {

         Ok(val) => val ,

         Err(string_error) => return Err(string_error)

               
 
  };



   // -----------------------------------------------------------



 print!("RENDA ATUAL R$    _ ") ; 

  let _campo3 = entry_str_renda()?;
  

 
        // ... < funcao de avaliacao de comma + exception ... >

    

   let convert_renda : f64 = match get_type_is_comma(&_campo3) {


           Ok(val) => val , 

             Err(parse_exception_float) => {

                      panic!("ParseException  :  {} " , parse_exception_float);
             },

   }; 


  // -----------------------------------------------------------


   println!("\n");

   print!("IDADE  _  ") ; 

   let _campo4 = entry_str_idade()?;


   let _ = match is_correct_numeric(&_campo4) {


            Ok(val) => val , 

            Err(string_error)  => return Err(string_error)

   };


 

   // conversao


  let int_conv_idade : i16 = match int16_parse_convert(&_campo4) {


         Ok(val) => val ,

         Err(parse_exception) => {

                panic!("ParseIntException : Abort Critic Error by failed conversion  :  {} " , parse_exception);

         }, 


  };


  let _ = match is_idade_limits_currency(int_conv_idade) {


            Ok(val) => val, 

            Err(account_error) => {


                   return Err(account_error);
            },

  };



    // -----------------------------------------------------------
 

  println!("\n\n");
  

  println!("\n [ ------------ < ATA DE INVESTIMENTO > ---------------- ]");

  println!("\n");

 
  print!("ANO DE CONCESSAO - NOVO PERFIL  _  ") ; 

  let _campo5 = entry_str_ano_concessao()?; 


  let _ = match is_correct_numeric(&_campo5) {


            Ok(val) => val , 

            Err(string_error)  => return Err(string_error)

   };




   // _ convert _ 

   let conv_ano_concessao : i16 = match int16_parse_convert(&_campo5) {


         Ok(val) => val ,

         Err(parse_exception) => {

                panic!("ParseIntException : Abort Critic Error by failed conversion  :  {} " , parse_exception);

         }, 


  };

  

  // -----------------------------------------------------------



  println!("\n\n");

  // !! __ < chama a funcao de iteracao dos valores formatados > ___ 

 
  for it in iter() {


          println!("OPCAO   -  {}  :  {:?} "  , it.clone() as i8 , it.clone());
  }
  

  println!("\n");

  print!("OPCAO   _  ");


  let _campo6 = entry_str_enum_carteira()?;



   let _ = match is_correct_numeric(&_campo6) {


            Ok(val) => val , 

            Err(string_error)  => return Err(string_error)

   };



   // _______ << conversao >> _________ 


   let opcao_sistema : i8 = match inti8_parse_convert(&_campo6) {
 


         Ok(val) => val ,

         Err(parse_exception) => {

                panic!("ParseIntException : Abort Critic Error by failed conversion  :  {} " , parse_exception);

         }, 



   };  



 // -------------------------------------------------------------------------;;



  // _ campo institucional : aporte de investimento x requisitos para a compra de acao

  
  println!("\n");

  print!("APORTE PARA A COMPRA  R$ _  "); 

  let _campo7 = entry_str_aporte()?;


  
  
   let convert_aporte : f64 = match get_type_is_comma(&_campo7) {


           Ok(val) => val , 

             Err(parse_exception_float) => {

                      panic!("ParseException  :  {} " , parse_exception_float);
             },

   }; 
       
  



   
   // -------------------------------------------------------------------------;; 


   println!("\n");

   print!("PROJECAO - MESES   _  ") ; 

   let _campo8 = entry_str_meses_projecao()?;


   let _ = match is_correct_numeric(&_campo8) {

         Ok(val) => val ,

         Err(string_error) => return Err(string_error)

               
 
  };


  // conversao


  let conv_projecao_meses : i16 = match int16_parse_convert(&_campo8) {


         Ok(val) => val ,

         Err(parse_exception) => {

                panic!("ParseIntException : Abort Critic Error by failed conversion  :  {} " , parse_exception);

         },


  };







 
 
   // _ ----------------------------------------------------------------------------------------------------- 


   // converte a opcao desejada para enum 

         
   let convert_opcao_enum = <enum_carteiras_investimento::CarteirasAcoesInvestidor as std::convert::TryFrom<i8>>
   
    ::try_from(opcao_sistema)?;

   
   // !! funcao feita , pelo escopo de trait , importando a biblioteca , onde somente aceito pelo compilador

        

   // encapsula os itens

   let obj_class_investidor = Investidor::new(int_conv_sics , _campo2 , convert_renda , int_conv_idade);


   let obj_class_investidor_perfil = PerfilInvestidorAcoes::new(conv_ano_concessao , &obj_class_investidor , 

       &convert_opcao_enum , convert_aporte);


 

   /*

       
        <'_ informativo de algumas coisas _ '>

        1. Avalia a renda financeira 

        
        2. Enum , mostrando o valor de cada acao selecionada


        3. Entrada do valor 


        4. Validação do valor inserido , na proposta de compra da ação

   */


  

 
   let _ = obj_class_investidor_perfil.get_avaliacao_renda()?;

  

   println!("\n");


   let _ = obj_class_investidor_perfil.get_classificacao_acao_inv()?;

    


   
   println!("\n"); 


   println!("VALOR DA AÇÂO REQUISITORIA    _  {:2}"  , obj_class_investidor_perfil.get_entrada_acao_investimento()?); 


   println!("\n");


   let _  = obj_class_investidor_perfil.get_requisited_by_currency_value()?;



   println!("\n"); 

   // <'_  mostra aqui , o indice de projecao final do valor que o usuario verá , de 6 a 24 meses '_>

   let _ = obj_class_investidor_perfil.get_margem_rentabilidade(conv_projecao_meses)?;








    io::stdout().flush()?;

    io::stdin().read(&mut [0u8])?;

    Ok(())
 

 } // end_function < _ < 
  


 




 // _______ << validations >> ______________
 

 type TStringExceptionValue<T> = Result<T , Box<dyn std::error::Error>>; 




 pub fn is_correct_letters(x : &str) -> TStringExceptionValue<String> {


         // ___ está vazio ? __ 

         if x.is_empty() {

                return Err("String.Error :: o campo está vazio!".into());
    
         }



         let _trimmer = x.trim();



         if _trimmer.chars().any(|c| c.is_numeric()) {


                 return Err("String.Error : Atenção ! Não coloque números em letras".into());

         }




       Ok(_trimmer.to_string())  // result <+String + convert> 
 }



  //  -------------------- << validacao de numeros >> --------------------------



 pub fn is_correct_numeric(x : &str) -> TStringExceptionValue<String> {


         // ___ está vazio ? __ 

         if x.is_empty() {

                return Err("String.Error :: o campo está vazio!".into());
    
         }



         let _trimmer = x.trim();



         if _trimmer.chars().any(|c| c.is_alphabetic()) {


                 return Err("String.Error : Atenção ! Caracteres nao serão permitidos em numeros!".into());

         }




       Ok(_trimmer.to_string())  // result <+String + convert> 
 }






 // << conversao de valores >>

 use std::num::{ParseIntError}; 

 pub fn int64_parse_convert(x : &str) -> Result<i64 , ParseIntError> {

            x.trim().parse::<i64>()

 }

 
 pub fn int16_parse_convert(x : &str) -> Result<i16 , ParseIntError> {

            x.trim().parse::<i16>()

 }

 

 pub fn inti8_parse_convert(x : &str) -> Result<i8 , ParseIntError> {

            x.trim().parse::<i8>()

 }

 


 pub fn is_idade_limits_currency(age : i16) -> Result<() , Box<dyn std::error::Error>> 

 {

        // _ < validação de idade _>


        if age < 18 {


                return Err("ExceptionByAge : Idade não atende aos requisitos , para compra de ação !".into()) 
        }



    Ok(())

 }







 /*
 
       __ conversao e entrada de virgulas __ 
 
  */

  use stacktrace_exceptions::StackTraceException;

  pub fn get_type_is_comma(x : &str) -> Result<f64 , StackTraceException> {


           let _trimmer = x.trim();


           if _trimmer.is_empty() {

                  // devolve a acao dentro do padrão result da Exception 

                  return Err(StackTraceException::MEMORY_ARITHMETIC_EXCEPTION);  // invoca a exception de memoria
           }



           

           // __ (( modelo novo de validacao )) __ 

           _trimmer.replace("," , ".").parse::<f64>() 

           .map_err(|_| StackTraceException::FLOAT_EXCEPTION_ECONVERT)



  }








 // << exceptions >> 


 pub mod stacktrace_exceptions {


       // ... [ classe gerenciadora da excecao e cenários ] ... 

       #[derive(Debug , Clone , Eq , PartialEq)] 

       pub enum StackTraceException {

                 
                 MEMORY_ARITHMETIC_EXCEPTION ,

                 INVALID_ARGUMENT_EXCEPTION , 

                 FLOAT_EXCEPTION_ECONVERT,     

                 FAILED_REQUISITED_BY_ACCOUNT        // para erro seletivo de renda !<>      
       }



       use std::fmt;

       // --------------- < formatação enum + fmt + erro > -----------------------

       impl fmt::Display for StackTraceException {


                 fn fmt(&self , f : &mut fmt::Formatter<'_>) -> fmt::Result {


                            let _errormessage = match self {


                                   Self::MEMORY_ARITHMETIC_EXCEPTION => "MemoryException : Falha na memória de cálculo de valores ou geração de faturamento de compra", 

                                   Self::INVALID_ARGUMENT_EXCEPTION => "Invalid.Error <int> : Falha na entrega de valores", 

                                   Self::FLOAT_EXCEPTION_ECONVERT => "Error ! Falha na conversão float ou e-parse<0,00> ",

                                   Self::FAILED_REQUISITED_BY_ACCOUNT => "Requisit By Value Exception : Não atende aos requisitos ,  como renda aplicada , saldo insuficiente de compra do tiulo ou idade nao permitida" , 
                                  


                            };



                            // _ formata com a macro

                            write!(f , "{}" , _errormessage) 

                 }    

              
       } 


       impl std::error::Error for StackTraceException {}  // ----------- << classifica a enum com erro Exception >> --------------



 }




 // ___________ entrada de dados ____________


 pub fn entry_str_sics() -> TStringExceptionValue<String> {

        let mut entry_str_sics : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_sics)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_sics.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }

        



    Ok(_trimmer.to_string())


 }



// _________________________________________________________


 pub fn entry_str_nome() -> TStringExceptionValue<String> {

        let mut entry_str_nome : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_nome)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_nome.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }


        else if _trimmer.len() < 10 {

              return Err("String.Alert : Quantidade insuficiente de caracteres".into())
        }

        



    Ok(_trimmer.to_string())


 }



 // _________________________________________________________


 pub fn entry_str_renda() -> TStringExceptionValue<String> {

        let mut entry_str_renda : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_renda)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_renda.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }


        
        



    Ok(_trimmer.to_string())


 }


  // _________________________________________________________


 pub fn entry_str_idade() -> TStringExceptionValue<String> {

        let mut entry_str_idade : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_idade)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_idade.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }

        
        
        



    Ok(_trimmer.to_string())


 }             






  // _________________________________________________________


 pub fn entry_str_ano_concessao() -> TStringExceptionValue<String> {

        let mut entry_str_ano_concessao : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_ano_concessao)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_ano_concessao.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }

        
        
        



    Ok(_trimmer.to_string())


 }             



 // -------------- [ string to enum option ]----------------------------


 pub fn entry_str_enum_carteira() -> TStringExceptionValue<String> {

        let mut entry_str_enum_carteira : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_enum_carteira)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_enum_carteira.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }

        
        
        



    Ok(_trimmer.to_string())


 }             
 


  // _______________  [ ]  __________________________

  
  

 pub fn entry_str_aporte() -> TStringExceptionValue<String> {

        let mut entry_str_aporte : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_aporte)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_aporte.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }


        
        



    Ok(_trimmer.to_string())


 }






 
  // <'_ campo extra : meses de projecao do aporte _ '> 

   pub fn entry_str_meses_projecao() -> TStringExceptionValue<String> {

        let mut entry_str_meses_projecao : String = String::new();

        // << entrda de dados >> 

        io::stdout().flush()?; 

        io::stdin().read_line(&mut entry_str_meses_projecao)?;


        // retiram-se os espaços 

        let _trimmer = entry_str_meses_projecao.trim();

       
        if _trimmer.is_empty() {

              return Err("String.Error :: o campo está vazio!".into())
        }


        
        



    Ok(_trimmer.to_string())


 }