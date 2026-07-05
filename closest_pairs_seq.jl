using Random

mutable struct Point
    x::Float64
    y::Float64
end

function compareX(a::Point, b::Point)
    return a.x - b.x
end

function compareY(a::Point, b::Point)
    return a.y - b.y
end

function dist(p1::Point, p2::Point)
    return sqrt((p1.x - p2.x)^2 + (p1.y - p2.y)^2)
end

function bruteForce(P, n)
    min_dist = Inf
    for i in 1:n
        for j in (i+1):n
            if dist(P[i], P[j]) < min_dist
                min_dist = dist(P[i], P[j])
            end
        end
    end
    return min_dist
end

function stripClosest(strip, size, d)
    min_dist = d
    sort!(strip, by=point -> point.y)

    for i in 1:size
        for j in (i+1):size
            if strip[j].y - strip[i].y >= min_dist
                break
            end
            if dist(strip[i], strip[j]) < min_dist
                min_dist = dist(strip[i], strip[j])
            end
        end
    end
    return min_dist
end

function closestUtil(P, n)
    if n <= 3
        return bruteForce(P, n)
    end

    mid = div(n, 2)
    midPoint = P[mid]
    dl = closestUtil(P[1:mid], mid)
    dr = closestUtil(P[(mid+1):end], n - mid)
    d = min(dl,dr)
    strip = Point[]

    for i in 1:n
        if abs(P[i].x - midPoint.x) < d
            push!(strip, P[i])
        end
    end

    return min(d, stripClosest(strip, length(strip), d))
end

function closest(P)
    P = sort(P, by=point -> point.x)
    return closestUtil(P, length(P))
end

P = [Point(rand(), rand()) for _ in 1:10000000]

function main()
    println("The smallest distance is ", closest(P))
end

@time main()