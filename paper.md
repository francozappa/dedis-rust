# TODOs #

TODO

# Title #

* Title: Towards High-Interaction Virtual ICS Honeypots-in-a-Box

* Keys: ICS, honeypot, SCADA, CTF, security, simulation, emulation

* Conference: CPS-SPC
    * max 12 pages double column
    * http://eecs.oregonstate.edu/cps-spc/index.html
    * deadline: **27 july**

# Abstract #

* ICS security major threat
    * several attacks (internet facing)
    * honeypot is one countermeasure

* present the design of ... Honeypot
    * a realistic, muli-device support (high-interaction),
    * low-cost, securely maintainable, (re)configurable, inspectable (virtual)
    * scalable, extensible (modular)
    * support multiple listening services (server)
    * physical processes, industrial protocols, time and determinism (ICS domain)
    * using a virtual, high-interaction, server honeypot

* implementation:
    * MiniCPS framework that combines lightweight virtualization, rich network
        emulation (link shaping) and physical
        process and devices simulation that runs on a single Linux kernel
    * Reach our goals

* novel implementation features:
    * ICS: basic support for Ethernet/IP
    * generic: first academic work that combines high-interaction and virtualization without using virtual
      machines

* evaluation:
    * replicate a real water testbed SWaT
    * in the context of CTF event

# Introduction #

* ICS security major threat
    * several attacks (internet facing)
    * traditional firewalls and NIDS

* honeypot are an effective, complementary countermeasure
    * why

* identify that little academic work has been done in the filed of ICS honeypot
    * explain why

* present a ICS honeypot design
    * targeting traditional and ICS requirements
    * using a virtual, high-interaction, server honeypot

* implementation:
    * MiniCPS framework that combines lightweight virtualization, rich network
        emulation (link shaping) and physical
        process and devices simulation that runs on a single Linux kernel
    * Reach our goals

* novel implementation features:
    * ICS: basic support for Ethernet/IP
    * generic: first academic work that combines high-interaction and virtualization without using virtual
      machines

* evaluation:
    * replicate a real water testbed SWaT
    * in the context of CTF event

* assumptions and/or limitations:
    * design against a relatively strong attacker model
    * focus on core honeypot functionalities eg: no data analytics
    * relatively small restricted evaluation period
    * focus on slow ICS (eg: water treatment)

* following sections references

# Background #

## ICS network ##

* FIG reuse modified from MiniCPS paper
    * list components
    * convergence of ICT and ICS network architecture and management
    * slow evolution of ICS/SCADA systems (especially software)

* ICS netsec cons
    * attacker surface: both physical and cyber
    * complex sw mix: proprietary legacy (insecure eg modbus) + free TCP/IP Ethernet stacks
    * no pentest, security audits, security certifications, virtualization
    * too many standards: country-based, industry-based
    * slow or impossible (legacy) to update/patch sw
    * expensive to encrypt and authenticate (ICS constraints)
    * real-time ICS cannot be rebooted  (wait till maintenance period)
    * difficult to do access control (eg: ACL)

## ICS honeypot ##

* honeypot def
    * historical context

* honeypot classif
    * with requirements
        * realistic, interactive, secure, scalable and stealthy
        * easy to configure, manage for a non-skilled operator
        * attacker surface (the more the better)
            * the larger it is the more realistic
            * the larger it is the more difficult is to confine the attacker
            * eg: attacker will spend more time on it
        * low-cost economical, memory, disk and cpu
    * real vs virtual vs hybrid
    * low vs high interaction
    * production vs research
    * client vs server

* ICS requirements
    * physical process-dependent
    * industrial protocol process-dependent
    * time constraints eg: real-time
    * deterministic: task and network pkts
    * same value can be checked using more than one reading
    * intermediate (encoded) values


* brief discussion and forward pointer to related work
    * honeynets
    * conpot

## MiniCPS ##

* What is MiniCPS[CITE]

* How basic building block work (no pic)

* Why MiniCPS can be used to build an honeypot
    * typical ICS/SCADA honeypots
        * lack of network sub-system
        * focus on particular device, typically PLCs and HMIs

* Why public API

# Design #

## Problem statement ##

* problem statement
    * a realistic, muli-device support (high-interaction design, server design),
    * low-cost, securely maintainable, (re)configurable, inspectable (virtual
        design)
    * scalable, extensible (modular implementation)
    * physical processes, industrial protocols, time and determinism (ICS domain)
    * running on a single Linux kernel (lightweight virtual implementation)

* given the ICT and ICS background
    * we didn't find what we wanted
    * we decided to implement it
    * according to a precise attacker model


## Attacker Model ##

* attacker is relatively powerful
    * come over the internet (shodan)
    * fingerprint ICS (`nmap`, `wireshark`, `xprobe2`)
    * speaks industrial protocol
    * familiar with automation eng

* attacker interfaces
    * network level: VPN server
    * control level: gateway running ssh server

* limitations
    * no multiple-attacks (login twice, two attacker same honeypot)
    * no skilled SCADA operator attack (multi-point attack)
    * no complex configs (multi-language support)

## Honeypot Architecture ##

* given honeypot classifications, our requirements and attacker model
    * enumerate design points: virtual, high-interaction, ICS, server
    * explain design decisions

* FIG architecture explaining the main components
    * similar to real ICS: emulated network, simulated pp and devices
    * attacker over the internet
    * realistic attacker interfaces: gateway and vpn

* physical separation (color coded)
    * vs logical one

* usage cases
    * production and research

* focus on core functionalities
    * no analytics

# Implementation #

## System overview ##

* novel implementation features:
    * ICS: basic support for Ethernet/IP
    * generic: first academic work that combines high-interaction and virtualization without using virtual
      machines

* We used MiniCPS, backward pointer to MiniCPS subsec

* FIG: honeypot zoom in
    * real network topology, IP addresses/netmask
    * real Enip packet (subset)
    * physical layer API between simulated pp and devices
    * real services running on devices: `gohead`, `ftpd`, etc
    * SDN controller under the hood

* Then we will focus on the key components in the following subsecs
    * VPN server
    * Gateway device
    * Network emulation
    * Simulated components
    * Data collection

## VPN ##

* real hw reference: Cisco firewall IOS
* target service: openvpn, cisco Anyconnect, ocserv
* vuln: weak credential
* interface: network
    * even traffic encrypted we can sniff traffic from the internal network
* implementation: `ocserv`

## Gateway ##

* real hw reference: 3g gateway
* target service: telnet or sshd
* vuln: weak credential
* interface: chrooted shell (process, filesystem isolation)

## Network emulation ##

* `lxc` lightweight virtualization and network isolation
* `tclink` shaping

## Simulated components ##

* PLC logic
    * same as spec

* Tanks for the physical process
    * Continuity equation and Bernoulli formula

* `cpppo` for enip subset

## Data collection ##

* command: syslog, auditd, keylogger
* network traffic: tcpdump, bro, snort

## Additional benefits ##

* Plus: hardware in the loop extension (hybrid)
* Plus: SDN architecture integration
* Plus: Parametric security level (both research and production)
* Plus: Honeynet extension (connect different MiniCPS instances)

# Evaluation #

## Evaluation Context ##

* CTF intro
    * what is
    * types
    * novel type of CTF targeted to ICS

* S3 organization
    * teams
    * scheduling: two sessions

* assumptions;
    * water treatment (slow ICS)
    * simulate a subset (only hydraulics)
    * Ethernet/IP industrial protocol
    * exposed only the ssh gateway interface

## Experimental Setup ##

* scoring setup
    * flask webapp
    * lets encrpgy HTTPS
    * credential based login
    * form-based submission: common errors, brute-force

* live help setup
    * email (challenge contains author filed)
    * IRC public s3 channel on freenode.org

* setup descriptions;
    * why on AWS EC2, develop 1 and replicate

* ssh config
    * attacker user and jail
    * use two servers on port 22 (port forwarding)
    * private keys distribution
    * port forwarding


## Results ##

* challenges descriptions;
    * scenarios, look at SOLUTION.md

* TABLE
    * TODO

* table description
    * TODO

* put lesson learned
    * `proc` file system hardening
    * automatic or attacker-based challenge restarts
    * effective descriptions and simple flags are key
    * eg: five vs 5
    * time zone may be a problem

# Related Works #

* TABLE
    * related work comparison
    * following pars accordingly

* design and implementation of ICS

* plc honeypots


## Frameworks ##

* IoTPOT
* Conpot
* Honeyd

## ICS Simulation ##

* UIUC paper

# Conclusions #

* future works
    * design
        * better automation: start, restart, stop
        * extension to real-time ICS: more difficult to manage
        * reuse of SDN
    * features
        * advanced logging capabilities eg: IDS or auditd
        * advanced data analytics
    * implementations
        * User Mode Linux (UML)
            * https://en.wikipedia.org/wiki/User-mode_Linux
            * http://www.landley.net/code/UML.html
    * evaluation
        * honeynet extension: more MiniCPS instances
        * WADI honeypot instance
        * publish industrial realistic public IP on shodan or ERIPP
        * advertise as attackable on pastebin, twitter, etc..
        * add hardware-in-the-loop


