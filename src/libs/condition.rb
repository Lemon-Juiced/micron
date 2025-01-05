# Micron's Condition Library

class Condition
    def self.greater(x, y)
        return x > y
    end
    def self.less(x, y)
        return x < y
    end
    def self.equal(x, y)
        return x == y
    end
    def self.greater_equal(x, y)
        return x >= y
    end
    def self.less_equal(x, y)
        return x <= y
    end
    def self.not_equal(x, y)
        return x != y
    end
end