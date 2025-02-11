import { BN } from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { PublicKey } from "@solana/web3.js"
import { startAnchor } from "solana-bankrun"
import { BankrunProvider } from "anchor-bankrun"
import { Voting } from "../target/types/voting"

import IDL from "../target/idl/voting.json"
import { expect } from "chai"

// const IDL = require("../target/idl/voting.json")

const votingAddress = new PublicKey("FhnUQ3mgYLTuLV7RZQaX4WMgnvigUoL4rKF8nH8PfqVc")

describe("voting", () => {
    let context;
    let provider;
    let votingProgram;

    before(async () => {
        context = await startAnchor(
            "",
            [{ name: "voting", programId: votingAddress }],
            []
        )

        provider = new BankrunProvider(context)

        votingProgram = new Program<Voting>(
            IDL as any,
            provider
        )
    })



    it("initialize poll", async () => {

        await votingProgram.methods.initializePoll(
            new BN(1),
            "what is your hobby",
            new BN(0),
            new BN(1000 * 60 * 60)
        ).rpc()

        const [pollAddress] = PublicKey.findProgramAddressSync(
            [new BN(1).toArrayLike(Buffer, "le", 8)],
            votingAddress
        )

        const poll = await votingProgram.account.poll.fetch(pollAddress)

        console.log("poll = ", poll)

        expect(poll.pollId.toNumber()).equal(1)
        expect(poll.description).equal("what is your hobby")
        expect(poll.pollStart.toNumber()).equal(0)
        expect(poll.pollEnd.toNumber()).equal(1000 * 60 * 60)
    })

    it("initialize candidate", async () => {
        await votingProgram.methods.initializeCandidate(
            "liu",
            new BN(1)
        ).rpc()

        await votingProgram.methods.initializeCandidate(
            "wang",
            new BN(1)
        ).rpc()

        const [liuAddress] = PublicKey.findProgramAddressSync(
            [new BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("liu")],
            votingAddress
        )
        const [wangAddress] = PublicKey.findProgramAddressSync(
            [new BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("wang")],
            votingAddress
        )

        const liuCandidate = await votingProgram.account.candidate.fetch(liuAddress)
        const wangCandidate = await votingProgram.account.candidate.fetch(wangAddress)

        console.log("liuCandidate = ", liuCandidate)
        console.log("wangCandidate = ", wangCandidate)

        expect(liuCandidate.candidateName).to.equal("liu");
        expect(liuCandidate.candidateVotes.toNumber()).to.equal(0)
        expect(wangCandidate.candidateName).to.equal("wang");
        expect(wangCandidate.candidateVotes.toNumber()).to.equal(0)
    })

    it("vote", async () => {

        await votingProgram.methods.vote(
            "liu",                      // candidate_name
            new BN(1)                   // poll_id
        ).rpc()

        const [liuAddress] = PublicKey.findProgramAddressSync(
            [new BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("liu")],
            votingAddress
        )

        const liuCandidate = await votingProgram.account.candidate.fetch(liuAddress)

        console.log("liuCandidate = ", liuCandidate)
        
        expect(liuCandidate.candidateVotes.toNumber()).to.equal(1)
    })
});
