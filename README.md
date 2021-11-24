### WIP project to build a bounty chain using Substrate


                                                +-----------+          +-----------+ +-------+                        +-----------+
                                               | Substrate |          | Frontier  | | IPFS  |                        | Frontend  |
                                               +-----------+          +-----------+ +-------+                        +-----------+
                                                     |--------------------\ |           |                                  |
                                                     || EVM compatibility |-|           |                                  |
                                                     ||-------------------| |           |                                  |
                         --------------------------\ |                      |           |                                  |
                         | pallet_tx_asset_payment |-|                      |           |                                  |
                         |-------------------------| |                      |           |                                  |
                                                     |                      |           |                                  |
                                                     | Accepts multi currency deposits  |                                  |
                                                     |-------------------------------------------------------------------->|
                                                     |                      |           |                                  |
                                                     |                      |           |                      Lesson bank |
                                                     |                      |           |<---------------------------------|
                                 ------------------\ |                      |           |                                  |
                                 | pallet_bounties |-|                      |           |                                  |
                                 |-----------------| |                      |           |                                  |

                                 ------------------\ |                      |           |                                  |
                                 | pallet_treasury |-|                      |           |                                  |
                                 |-----------------| |                      |           |                                  |
                               --------------------\ |                      |           |                                  |
                               | pallet_collective |-|                      |           |                                  |
                               |-------------------| |                      |           |                                  |
                                -------------------\ |                      |           |                                  |
                                | pallet_society   |-|                      |           |                                  |
                                |------------------| |                      |           |                                  |
                                                     |                      |           | ---------------------------\     |
                                                     |                      |           |-| Image library for badges |     |
                                                     |                      |           | |--------------------------|     |
                                                     |                      |           |                                  |
                                                     | generate unique badge            |                                  |
                                                     |--------------------------------->|                                  |
                                                     |                      |           |                                  |
                                                     |                      |           |                Substrate Connect |
                                                     |<--------------------------------------------------------------------|
                                                     |                      |           |                                  |
                                                     

<details>
  <summary>
[Generate ascii diagram](https://textart.io/sequence) using: 
  </summary>
object Substrate Frontier IPFS Frontend 

note left of Frontier: EVM compatibility 
 
note left of Substrate: pallet_tx_asset_payment 

Substrate->Frontend: Accepts multi currency deposits 

Frontend->IPFS: Lesson bank

note left of Substrate: pallet_bounties

note right of Frontend: Society UX

note right of Frontend: Council UX

note right of Frontend: Bounty UX

note left of Substrate: pallet_treasury 

note left of Substrate: pallet_collective 

note left of Substrate: pallet_society (contributors and bounty hunters) 

note right of IPFS: Image library for badges 

Substrate->IPFS: generate unique badge 

Frontend->Substrate: Substrate Connect
  </details>
