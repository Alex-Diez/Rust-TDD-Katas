package game;

public class Game {

    private int score = 0;

    public void rollBall(int point) {
        score += point;
    }

    public int score() {
        return score;
    }
}
