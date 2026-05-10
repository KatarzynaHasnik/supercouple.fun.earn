export default function Profile() {

  return (

    <div className="bg-[#111827] rounded-xl p-6 w-[360px]">

      <h2 className="text-white text-2xl font-bold">
        Reputation Profile
      </h2>

      <div className="mt-6 text-cyan-400 text-5xl font-bold">
        87
      </div>

      <p className="text-gray-400 mt-2">
        Trust Score
      </p>

      <div className="mt-6 space-y-3 text-gray-300">

        <div className="flex justify-between">
          <span>Completed</span>
          <span>24</span>
        </div>

        <div className="flex justify-between">
          <span>Failed</span>
          <span>2</span>
        </div>

        <div className="flex justify-between">
          <span>Tier</span>
          <span>Verified</span>
        </div>

      </div>

    </div>
  );
}