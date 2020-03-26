# Human Knowledge Protocol - HKP

Current world has a an overload of information. Traditional media and social media generate news and information on a rate that nobody is able to process in terms of fact checking and accounting for inherent biases by authors. False news and distortion of facts happens on purpose or by accident and generate huge problems. Universal knowledge protocol aims to generate curated facts by using voting mechanisms and credibility scores. An implementation of this protocol is intended by leveraging decentralised and immutable IOTA ledger. Building on top of IOTA allows for using existing research, such as UIP (Unified Identity Protocol) and MAM (Masked Authenticated Messaging) streams.

## From raw information to reasonable facts
All observations inherently include the viewpoint of the observer. This is the case even if its impact is deliberately minimized. In order to evaluate raw information, the object and the subject have to be separated. Later on as information is retrieved, the information retrieving entity constructs information from its own viewpoint. For this purpose the raw information is separeted into statement records and author records. A statement record contains the actual information as observed by the author, reasoning behind the fact and some additional information that is adressed later in this document. An author record is very similar to a statement record; difference is that it defines an entity that is responsible for its own actions in the database. In addition to these, vote records are issued by authors to tell their opinions on any other records.

Rules
### Record
1.	A record is a database entry that can not be modified or erased.
2.	Each record must be signed by an author.
3.	A revision on a record can be issued by its original author. This does not remove the previous record, but invalidates it.
### Statement record
4.	All rules of records apply to statement records.
5.	A statement is an observation about reality and its construction.
6.	A statement must be backed up by reasoning that was used to create it. This can include (and is not limited to) logic used, external references and links to other records.
### Author record
7.	All rules of records apply to author records.
8.	An author record verifies that issued records are truly issued by said author.
### Vote record
9.	All rules of record apply to vote records
10.	Only one vote for a individual record from an author is considered valid.
11.	Only the latest revision of vote is considered valid.
12.	Author cannot vote on their own records
13.	A vote issues a bayesian probability in range of [0.0 ... 1.0] for a record. Score of ’0’ signals distrust against the record, ’0.5’ undecided and ’1’ totally agrees with the record.
### Viewpoint
14.	A viewpoint is a personalized way of viewing and evaluating records. This can include tools as weights for authors by their credibility scores, author/community maintained whitelists/blacklists and personal preferences.
15.	A bayesian probability in range [-1..1] is calculated for every record. This can be considered the credibility of the record from this viewpoint.

## Why IOTA as database backend?

Implementing HKP in IOTA tangle gives many advantages over traditional centralized database solution:
* Immutability: No-one can alter existing records or fake the database contents. Even authors themselves can only issue revisions on their previous records.
* Accountability: Every action in the database can be traced down to the author responsible for it.
* Attack resistance: Decentraliced nature of the database is very robust against entities (goverments, corporations, influencers) trying to disable it as there is no central server that can be shut down.
* No fees: Contribution to the database does not rely on financial situation
* IOTA token: Built in financial token to use in incentivizing and rewarding authors for quality content.
