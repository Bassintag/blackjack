import { PlayingCard } from "@/features/trainer/components/PlayingCard";
import { useTrainerState } from "@/features/trainer/hooks/useTrainerState";
import { TrainerButtons } from "../components/TrainerButtons";
import { TrainerStats } from "../components/TrainerStats";

const TrainerPage = () => {
  const { dealer, player } = useTrainerState();

  return (
    <div className="relative grow flex flex-col gap-3 p-3">
      <TrainerStats className="absolute right-3 top-2" />
      <div className="grow flex flex-col justify-center items-center my-auto gap-6">
        <div className="flex flex-col gap-1.5">
          <div className="text-center text-xl font-bold">Dealer</div>
          <PlayingCard value={dealer} />
        </div>
        <div className="flex flex-col gap-1.5">
          <div className="text-center text-xl font-bold">Your hand</div>
          <div className="flex flex-row gap-3">
            {player.map((card, i) => (
              <PlayingCard key={i} value={card} />
            ))}
          </div>
        </div>
      </div>
      <TrainerButtons />
    </div>
  );
};

export const element = <TrainerPage />;
