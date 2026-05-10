export default function BountyCard() {

  return (

    <div className="bg-[#111827] border border-cyan-400 rounded-xl p-6 w-[360px]">

      <h2 className="text-white text-2xl font-bold">
        Proof of Vibe Challenge
      </h2>

      <p className="text-gray-400 mt-3">
        Complete a creative mission together and build reputation through action.
      </p>

      <div className="mt-6">

        <div className="flex justify-between text-sm text-gray-300">
          <span>Stake</span>
          <span>0.05 SOL</span>
        </div>

        <div className="flex justify-between text-sm text-gray-300 mt-2">
          <span>Reward</span>
          <span>0.10 SOL</span>
        </div>

      </div>

      <button
        className="mt-6 bg-cyan-400 text-black px-4 py-3 rounded-lg w-full font-semibold"
      >
        Accept & Stake
      </button>

    </div>
  );
}